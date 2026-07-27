/**
 * CueForge Studio — Interactive Web UI Simulation Engine
 * Phase 1 Core Physics & Visual Renderer
 */

const CanvasWidth = 1000;
const CanvasHeight = 520;

// Table specifications (9ft regulation ratio: bed width 1.27m, length 2.54m)
const TableWidthMeters = 1.27;
const TableLengthMeters = 2.54;

const CushionThickness = 32; // Canvas pixels
const BedX = CushionThickness;
const BedY = CushionThickness;
const BedWidth = CanvasWidth - 2 * CushionThickness;
const BedHeight = CanvasHeight - 2 * CushionThickness;

const ScalePX = BedWidth / TableLengthMeters; // Pixels per meter

const BallRadiusMeters = 0.028575; // 2.25 inch diameter
const BallRadiusPX = BallRadiusMeters * ScalePX;
const BallMass = 0.170; // kg

const RestitutionBall = 0.95;
const RestitutionRail = 0.85;
const Gravity = 9.81;
const SlidingFrictionCoef = 0.20;
const RollingFrictionCoef = 0.015;
const SpinningFrictionCoef = 0.005;

const PocketRadiusPX = 24;

// Ball Colors
const BallColors = {
  0: '#ffffff', // Cue Ball
  1: '#facc15', // 1-Yellow
  2: '#1d4ed8', // 2-Blue
  3: '#b91c1c', // 3-Red
  4: '#6b21a8', // 4-Purple
  5: '#c2410c', // 5-Orange
  6: '#15803d', // 6-Green
  7: '#a16207', // 7-Maroon
  8: '#18181b', // 8-Black
  9: '#fde047', // 9-Striped Yellow
};

class Vector2 {
  constructor(x = 0, y = 0) {
    this.x = x;
    this.y = y;
  }

  add(v) { return new Vector2(this.x + v.x, this.y + v.y); }
  sub(v) { return new Vector2(this.x - v.x, this.y - v.y); }
  mul(s) { return new Vector2(this.x * s, this.y * s); }
  div(s) { return new Vector2(this.x / s, this.y / s); }
  dot(v) { return this.x * v.x + this.y * v.y; }
  lenSq() { return this.x * this.x + this.y * this.y; }
  len() { return Math.sqrt(this.lenSq()); }
  normalize() {
    const l = this.len();
    return l > 1e-9 ? this.div(l) : new Vector2(0, 0);
  }
  dist(v) { return this.sub(v).len(); }
}

class Ball {
  constructor(id, x, y) {
    this.id = id;
    this.pos = new Vector2(x, y); // Canvas pixels
    this.vel = new Vector2(0, 0); // Pixels per second
    this.angVel = { x: 0, y: 0, z: 0 }; // rad/s
    this.active = true;
    this.state = 'Stationary'; // 'Stationary', 'Sliding', 'Rolling', 'Pocketed'
  }

  isMoving() {
    return this.active && (this.state === 'Sliding' || this.state === 'Rolling');
  }
}

class CueForgeSimulation {
  constructor(canvas) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d');
    this.balls = [];
    this.aimAngle = 0; // Radians
    this.cuePower = 2.5; // m/s
    this.cueElevation = 0; // Degrees
    this.spinOffsetX = 0; // [-1.0, 1.0]
    this.spinOffsetY = 0; // [-1.0, 1.0]

    this.isSimulating = false;
    this.isDraggingAim = false;
    this.isBallInHand = false;

    this.pockets = [
      new Vector2(BedX, BedY),                             // Top Left
      new Vector2(BedX + BedWidth / 2, BedY - 6),           // Top Center
      new Vector2(BedX + BedWidth, BedY),                  // Top Right
      new Vector2(BedX, BedY + BedHeight),                  // Bottom Left
      new Vector2(BedX + BedWidth / 2, BedY + BedHeight + 6),// Bottom Center
      new Vector2(BedX + BedWidth, BedY + BedHeight),       // Bottom Right
    ];

    this.stats = { shots: 0, pots: 0, fouls: 0 };
    this.eventLogs = [];

    this.initNineBallRack();
    this.bindEvents();
    this.startLoop();
  }

  initNineBallRack() {
    this.balls = [];
    const cueX = BedX + BedWidth * 0.25;
    const cueY = BedY + BedHeight * 0.5;
    this.balls.push(new Ball(0, cueX, cueY));

    const rackX = BedX + BedWidth * 0.72;
    const rackY = BedY + BedHeight * 0.5;
    const r = BallRadiusPX;
    const dx = r * 1.732; // sqrt(3) * r

    // 9-Ball Diamond Rack layout
    const positions = [
      { id: 1, x: rackX, y: rackY },
      { id: 2, x: rackX + dx, y: rackY - r },
      { id: 9, x: rackX + dx, y: rackY + r }, // 9 in middle
      { id: 3, x: rackX + 2 * dx, y: rackY - 2 * r },
      { id: 8, x: rackX + 2 * dx, y: rackY },
      { id: 4, x: rackX + 2 * dx, y: rackY + 2 * r },
      { id: 5, x: rackX + 3 * dx, y: rackY - r },
      { id: 6, x: rackX + 3 * dx, y: rackY + r },
      { id: 7, x: rackX + 4 * dx, y: rackY },
    ];

    for (const p of positions) {
      this.balls.push(new Ball(p.id, p.x, p.y));
    }

    this.aimAngle = 0;
    this.isBallInHand = false;
    this.logEvent('9-Ball rack set. Aim line ready.', 'info');
    this.updateAIDetails();
  }

  respawnCueBall(customPos = null) {
    let cueBall = this.balls.find(b => b.id === 0);
    if (!cueBall) {
      cueBall = new Ball(0, 0, 0);
      this.balls.unshift(cueBall);
    }

    let spawnX = BedX + BedWidth * 0.25;
    let spawnY = BedY + BedHeight * 0.5;

    if (customPos) {
      const minX = BedX + BallRadiusPX + 5;
      const maxX = BedX + BedWidth - BallRadiusPX - 5;
      const minY = BedY + BallRadiusPX + 5;
      const maxY = BedY + BedHeight - BallRadiusPX - 5;

      spawnX = Math.max(minX, Math.min(maxX, customPos.x));
      spawnY = Math.max(minY, Math.min(maxY, customPos.y));
    }

    // Verify non-overlapping with active balls
    let attempts = 0;
    while (attempts < 30) {
      let overlap = false;
      for (const b of this.balls) {
        if (b.id !== 0 && b.active) {
          if (new Vector2(spawnX, spawnY).dist(b.pos) < BallRadiusPX * 2 + 2) {
            overlap = true;
            break;
          }
        }
      }
      if (!overlap) break;
      spawnY += BallRadiusPX * 2.2;
      if (spawnY > BedY + BedHeight - BallRadiusPX) {
        spawnY = BedY + BallRadiusPX + 10;
        spawnX += BallRadiusPX * 2.2;
      }
      attempts++;
    }

    cueBall.pos = new Vector2(spawnX, spawnY);
    cueBall.vel = new Vector2(0, 0);
    cueBall.active = true;
    cueBall.state = 'Stationary';

    this.isBallInHand = true;
    document.getElementById('shot-status').innerText = 'Ball in Hand (Click table to place)';
    document.getElementById('shot-status').style.color = '#3b82f6';
    this.logEvent('Cue ball returned to table (Ball in Hand). Click table to position.', 'info');
    this.updateAIDetails();
  }

  bindEvents() {
    this.canvas.addEventListener('mousedown', (e) => {
      if (this.isSimulating) return;

      const rect = this.canvas.getBoundingClientRect();
      const mx = e.clientX - rect.left;
      const my = e.clientY - rect.top;

      if (this.isBallInHand) {
        this.respawnCueBall({ x: mx, y: my });
        this.isBallInHand = false;
        document.getElementById('shot-status').innerText = 'Ready for Shot';
        document.getElementById('shot-status').style.color = '#10b981';
      } else {
        this.isDraggingAim = true;
        this.updateAimFromMouse(e);
      }
    });

    window.addEventListener('mousemove', (e) => {
      if (this.isSimulating) return;
      const rect = this.canvas.getBoundingClientRect();
      const mx = e.clientX - rect.left;
      const my = e.clientY - rect.top;

      if (this.isBallInHand) {
        this.respawnCueBall({ x: mx, y: my });
      } else if (this.isDraggingAim) {
        this.updateAimFromMouse(e);
      }
    });

    window.addEventListener('mouseup', () => {
      this.isDraggingAim = false;
    });

    // Place Cue Ball / Ball in Hand Button
    document.getElementById('btn-place-cue').addEventListener('click', () => {
      this.respawnCueBall();
    });

    // Cue Strike Button
    document.getElementById('btn-strike').addEventListener('click', () => {
      this.fireCueStrike();
    });

    // Reset Rack Button
    document.getElementById('btn-reset-rack').addEventListener('click', () => {
      this.initNineBallRack();
    });

    // AI Hint Button
    document.getElementById('btn-ai-hint').addEventListener('click', () => {
      this.applyAIRecommendation();
    });

    // Sliders
    const powerSlider = document.getElementById('power-slider');
    powerSlider.addEventListener('input', (e) => {
      this.cuePower = parseFloat(e.target.value);
      document.getElementById('power-val').innerText = this.cuePower.toFixed(2);
    });

    const elevationSlider = document.getElementById('elevation-slider');
    elevationSlider.addEventListener('input', (e) => {
      this.cueElevation = parseInt(e.target.value);
      document.getElementById('elevation-val').innerText = this.cueElevation;
    });

    // Spin Target Drag
    const spinTarget = document.getElementById('spin-target');
    const spinCrosshair = document.getElementById('spin-crosshair');

    spinTarget.addEventListener('mousedown', (e) => {
      const updateSpin = (ev) => {
        const rect = spinTarget.getBoundingClientRect();
        let x = (ev.clientX - rect.left - rect.width / 2) / (rect.width / 2);
        let y = -(ev.clientY - rect.top - rect.height / 2) / (rect.height / 2);

        const len = Math.sqrt(x * x + y * y);
        if (len > 0.85) {
          x = (x / len) * 0.85;
          y = (y / len) * 0.85;
        }

        this.spinOffsetX = x;
        this.spinOffsetY = y;

        spinCrosshair.style.left = `${(x * 50 + 50)}%`;
        spinCrosshair.style.top = `${(-y * 50 + 50)}%`;

        document.getElementById('spin-x-val').innerText = x.toFixed(2);
        document.getElementById('spin-y-val').innerText = y.toFixed(2);
      };

      updateSpin(e);
      const onMove = (ev) => updateSpin(ev);
      const onUp = () => {
        window.removeEventListener('mousemove', onMove);
        window.removeEventListener('mouseup', onUp);
      };
      window.addEventListener('mousemove', onMove);
      window.addEventListener('mouseup', onUp);
    });

    document.getElementById('btn-reset-spin').addEventListener('click', () => {
      this.spinOffsetX = 0;
      this.spinOffsetY = 0;
      spinCrosshair.style.left = '50%';
      spinCrosshair.style.top = '50%';
      document.getElementById('spin-x-val').innerText = '0.00';
      document.getElementById('spin-y-val').innerText = '0.00';
    });
  }

  updateAimFromMouse(e) {
    const rect = this.canvas.getBoundingClientRect();
    const mx = e.clientX - rect.left;
    const my = e.clientY - rect.top;

    let cueBall = this.balls.find(b => b.id === 0);
    if (!cueBall || !cueBall.active) {
      this.respawnCueBall();
      cueBall = this.balls.find(b => b.id === 0);
    }

    if (cueBall && cueBall.active) {
      this.aimAngle = Math.atan2(my - cueBall.pos.y, mx - cueBall.pos.x);
      this.updateAIDetails();
    }
  }

  fireCueStrike() {
    if (this.isSimulating) return;

    let cueBall = this.balls.find(b => b.id === 0);
    if (!cueBall || !cueBall.active) {
      this.respawnCueBall();
      cueBall = this.balls.find(b => b.id === 0);
    }

    this.isBallInHand = false;

    // Apply impulse to cue ball
    const speedPX = this.cuePower * ScalePX;
    const dirX = Math.cos(this.aimAngle);
    const dirY = Math.sin(this.aimAngle);

    cueBall.vel = new Vector2(dirX * speedPX, dirY * speedPX);
    cueBall.state = 'Sliding';

    this.isSimulating = true;
    this.stats.shots += 1;
    document.getElementById('stat-shots').innerText = this.stats.shots;
    document.getElementById('shot-status').innerText = 'Shot Executing...';
    document.getElementById('shot-status').style.color = '#f59e0b';

    this.logEvent(`Cue struck (Speed: ${this.cuePower.toFixed(2)} m/s, Aim: ${(this.aimAngle * 180 / Math.PI).toFixed(1)}°)`, 'info');
  }

  applyAIRecommendation() {
    let cueBall = this.balls.find(b => b.id === 0);
    if (!cueBall || !cueBall.active) {
      this.respawnCueBall();
      cueBall = this.balls.find(b => b.id === 0);
    }

    const target = this.balls.find(b => b.id > 0 && b.active);
    if (!cueBall || !target) return;

    const targetPos = target.pos;
    const pocket = this.pockets[2]; // Top Right corner pocket

    const ghostX = targetPos.x - (pocket.x - targetPos.x) / targetPos.dist(pocket) * (2 * BallRadiusPX);
    const ghostY = targetPos.y - (pocket.y - targetPos.y) / targetPos.dist(pocket) * (2 * BallRadiusPX);

    this.aimAngle = Math.atan2(ghostY - cueBall.pos.y, ghostX - cueBall.pos.x);
    this.logEvent('AI Coach calculated ghost-ball aim angle.', 'info');
    this.updateAIDetails();
  }

  updateAIDetails() {
    const cueBall = this.balls.find(b => b.id === 0);
    const target = this.balls.find(b => b.id > 0 && b.active);

    if (cueBall && target) {
      document.getElementById('ai-target-ball').innerText = `Ball #${target.id}`;
      document.getElementById('ai-target-pocket').innerText = 'Corner Right';
      document.getElementById('ai-cut-angle').innerText = `${((this.aimAngle * 180 / Math.PI) % 90).toFixed(1)}°`;
    }
  }

  stepPhysics(dt) {
    let activeMovement = false;
    const g = Gravity;
    const mu_s = SlidingFrictionCoef;
    const mu_r = RollingFrictionCoef;

    // 1. Resolve pairwise ball-ball collisions
    for (let i = 0; i < this.balls.length; i++) {
      for (let j = i + 1; j < this.balls.length; j++) {
        const b1 = this.balls[i];
        const b2 = this.balls[j];

        if (!b1.active || !b2.active) continue;

        const delta = b2.pos.sub(b1.pos);
        const dist = delta.len();
        const minDist = 2 * BallRadiusPX;

        if (dist < minDist) {
          const normal = delta.normalize();
          const relVel = b2.vel.sub(b1.vel);
          const vNormal = relVel.dot(normal);

          if (vNormal < 0) {
            const impulse = -(1 + RestitutionBall) * vNormal / 2;
            b1.vel = b1.vel.sub(normal.mul(impulse));
            b2.vel = b2.vel.add(normal.mul(impulse));

            b1.state = 'Sliding';
            b2.state = 'Sliding';

            // Push apart to avoid overlap sticking
            const overlap = (minDist - dist) / 2;
            b1.pos = b1.pos.sub(normal.mul(overlap));
            b2.pos = b2.pos.add(normal.mul(overlap));

            this.logEvent(`Collision: Ball #${b1.id} <-> Ball #${b2.id}`, 'collision');
          }
        }
      }
    }

    // 2. Resolve cushion rail collisions & pocket capture
    for (const b of this.balls) {
      if (!b.active) continue;

      // Pocket Capture check
      for (let pIdx = 0; pIdx < this.pockets.length; pIdx++) {
        if (b.pos.dist(this.pockets[pIdx]) < PocketRadiusPX) {
          b.active = false;
          b.state = 'Pocketed';
          b.vel = new Vector2(0, 0);

          if (b.id === 0) {
            this.stats.fouls += 1;
            document.getElementById('stat-fouls').innerText = this.stats.fouls;
            this.logEvent('FOUL: Cue Ball scratched into pocket!', 'foul');
          } else {
            this.stats.pots += 1;
            document.getElementById('stat-pots').innerText = this.stats.pots;
            this.logEvent(`Ball #${b.id} pocketed!`, 'pocket');
          }

          const acc = this.stats.shots > 0 ? (this.stats.pots / this.stats.shots * 100).toFixed(0) + '%' : '0%';
          document.getElementById('stat-acc').innerText = acc;
        }
      }

      if (!b.active) continue;

      // Cushion Rail rebound
      const left = BedX + BallRadiusPX;
      const right = BedX + BedWidth - BallRadiusPX;
      const top = BedY + BallRadiusPX;
      const bottom = BedY + BedHeight - BallRadiusPX;

      if (b.pos.x < left) { b.pos.x = left; b.vel.x = -b.vel.x * RestitutionRail; }
      if (b.pos.x > right) { b.pos.x = right; b.vel.x = -b.vel.x * RestitutionRail; }
      if (b.pos.y < top) { b.pos.y = top; b.vel.y = -b.vel.y * RestitutionRail; }
      if (b.pos.y > bottom) { b.pos.y = bottom; b.vel.y = -b.vel.y * RestitutionRail; }

      // Friction integration
      const speed = b.vel.len();
      if (speed > 5) {
        activeMovement = true;
        const decel = (b.state === 'Sliding' ? mu_s : mu_r) * g * ScalePX;
        const newSpeed = Math.max(0, speed - decel * dt);

        if (newSpeed === 0) {
          b.vel = new Vector2(0, 0);
          b.state = 'Stationary';
        } else {
          b.vel = b.vel.mul(newSpeed / speed);
          b.pos = b.pos.add(b.vel.mul(dt));
        }
      } else {
        b.vel = new Vector2(0, 0);
        b.state = 'Stationary';
      }
    }

    if (!activeMovement && this.isSimulating) {
      this.isSimulating = false;

      // Check if cue ball was pocketed (scratched)
      const cueBall = this.balls.find(b => b.id === 0);
      if (!cueBall || !cueBall.active) {
        this.respawnCueBall();
      } else {
        document.getElementById('shot-status').innerText = 'Ready for Shot';
        document.getElementById('shot-status').style.color = '#10b981';
        this.logEvent('Shot complete. Table stationary.', 'info');
      }
    }
  }

  logEvent(msg, type) {
    const container = document.getElementById('event-log');
    const item = document.createElement('div');
    item.className = `log-item ${type}`;
    item.innerText = `[${new Date().toLocaleTimeString()}] ${msg}`;
    container.prepend(item);
  }

  draw() {
    const ctx = this.ctx;
    ctx.clearRect(0, 0, CanvasWidth, CanvasHeight);

    // 1. Draw Table Wooden Rail Frame
    ctx.fillStyle = '#2d1810';
    ctx.fillRect(0, 0, CanvasWidth, CanvasHeight);

    // Slate Cloth Bed
    ctx.fillStyle = '#0d5c3a';
    ctx.fillRect(BedX, BedY, BedWidth, BedHeight);

    // Cushion Borders
    ctx.strokeStyle = '#0a422a';
    ctx.lineWidth = 4;
    ctx.strokeRect(BedX, BedY, BedWidth, BedHeight);

    // Diamonds / Sights along rails
    ctx.fillStyle = '#f8fafc';
    for (let i = 1; i <= 3; i++) {
      ctx.beginPath(); ctx.arc(BedX + (BedWidth / 4) * i, BedY / 2, 3, 0, Math.PI * 2); ctx.fill();
      ctx.beginPath(); ctx.arc(BedX + (BedWidth / 4) * i, CanvasHeight - BedY / 2, 3, 0, Math.PI * 2); ctx.fill();
    }

    // 2. Draw Pockets
    for (const pocket of this.pockets) {
      ctx.fillStyle = '#111827';
      ctx.beginPath();
      ctx.arc(pocket.x, pocket.y, PocketRadiusPX, 0, Math.PI * 2);
      ctx.fill();
      ctx.strokeStyle = '#374151';
      ctx.lineWidth = 3;
      ctx.stroke();
    }

    // 3. Draw Aim Guide Line & Ghost Ball (if not simulating)
    const cueBall = this.balls.find(b => b.id === 0);
    if (cueBall && cueBall.active && !this.isSimulating) {
      const aimDirX = Math.cos(this.aimAngle);
      const aimDirY = Math.sin(this.aimAngle);

      // Raycast aim line to first obstacle
      let minRayDist = 800;
      for (const target of this.balls) {
        if (target.id === 0 || !target.active) continue;

        const toTarget = target.pos.sub(cueBall.pos);
        const projection = toTarget.x * aimDirX + toTarget.y * aimDirY;

        if (projection > 0) {
          const perpDistSq = toTarget.lenSq() - projection * projection;
          if (perpDistSq < Math.pow(2 * BallRadiusPX, 2)) {
            const d = projection - Math.sqrt(Math.pow(2 * BallRadiusPX, 2) - perpDistSq);
            if (d < minRayDist) {
              minRayDist = d;
            }
          }
        }
      }

      const ghostX = cueBall.pos.x + aimDirX * minRayDist;
      const ghostY = cueBall.pos.y + aimDirY * minRayDist;

      // Draw dashed aim line
      ctx.strokeStyle = 'rgba(255, 255, 255, 0.4)';
      ctx.lineWidth = 2;
      ctx.setLineDash([6, 6]);
      ctx.beginPath();
      ctx.moveTo(cueBall.pos.x, cueBall.pos.y);
      ctx.lineTo(ghostX, ghostY);
      ctx.stroke();
      ctx.setLineDash([]);

      // Draw Ghost Ball outline
      ctx.strokeStyle = 'rgba(255, 255, 255, 0.8)';
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.arc(ghostX, ghostY, BallRadiusPX, 0, Math.PI * 2);
      ctx.stroke();

      // Cue Stick graphic
      const cueLength = 220;
      const cueStartX = cueBall.pos.x - aimDirX * 30;
      const cueStartY = cueBall.pos.y - aimDirY * 30;
      const cueEndX = cueStartX - aimDirX * cueLength;
      const cueEndY = cueStartY - aimDirY * cueLength;

      ctx.strokeStyle = '#d97706';
      ctx.lineWidth = 6;
      ctx.beginPath();
      ctx.moveTo(cueStartX, cueStartY);
      ctx.lineTo(cueEndX, cueEndY);
      ctx.stroke();
    }

    // 4. Draw Balls
    for (const ball of this.balls) {
      if (!ball.active) continue;

      // Shadow
      ctx.fillStyle = 'rgba(0, 0, 0, 0.35)';
      ctx.beginPath();
      ctx.arc(ball.pos.x + 3, ball.pos.y + 4, BallRadiusPX, 0, Math.PI * 2);
      ctx.fill();

      // Ball Base
      ctx.fillStyle = BallColors[ball.id] || '#fff';
      ctx.beginPath();
      ctx.arc(ball.pos.x, ball.pos.y, BallRadiusPX, 0, Math.PI * 2);
      ctx.fill();

      // Highlights
      const grad = ctx.createRadialGradient(
        ball.pos.x - BallRadiusPX * 0.3,
        ball.pos.y - BallRadiusPX * 0.3,
        2,
        ball.pos.x,
        ball.pos.y,
        BallRadiusPX
      );
      grad.addColorStop(0, 'rgba(255, 255, 255, 0.6)');
      grad.addColorStop(0.5, 'rgba(255, 255, 255, 0)');
      grad.addColorStop(1, 'rgba(0, 0, 0, 0.3)');

      ctx.fillStyle = grad;
      ctx.beginPath();
      ctx.arc(ball.pos.x, ball.pos.y, BallRadiusPX, 0, Math.PI * 2);
      ctx.fill();

      // Ball Number Label (for non-cue ball)
      if (ball.id > 0) {
        ctx.fillStyle = '#ffffff';
        ctx.beginPath();
        ctx.arc(ball.pos.x, ball.pos.y, BallRadiusPX * 0.45, 0, Math.PI * 2);
        ctx.fill();

        ctx.fillStyle = '#1e293b';
        ctx.font = 'bold 11px Inter, sans-serif';
        ctx.textAlign = 'center';
        ctx.textBaseline = 'middle';
        ctx.fillText(ball.id.toString(), ball.pos.x, ball.pos.y + 1);
      }
    }
  }

  startLoop() {
    let lastTime = performance.now();
    const animate = (time) => {
      const dt = Math.min((time - lastTime) / 1000, 0.033);
      lastTime = time;

      if (this.isSimulating) {
        this.stepPhysics(dt);
      }

      this.draw();
      requestAnimationFrame(animate);
    };
    requestAnimationFrame(animate);
  }
}

// Initialize on DOM Ready
window.addEventListener('DOMContentLoaded', () => {
  const canvas = document.getElementById('table-canvas');
  window.cueForgeApp = new CueForgeSimulation(canvas);
});
