/**
 * CueForge Studio — Interactive Web UI Simulation Engine
 * Phase 1 Core Physics, HD Aiming Visualizer, and Settings Manager
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

const RestitutionBall = 0.95;
const RestitutionRail = 0.90;
const Gravity = 9.80665;

const PocketRadiusPX = 24;

// Cloth Presets (9ball_cloth_spec.md)
const ClothPresets = {
  simonis_860: {
    name: 'Simonis 860',
    material: '90% Wool / 10% Nylon',
    weave: 'Worsted (Napless)',
    baseMuR: 0.015,
    baseMuS: 0.20,
    color: '#0055a5', // Tournament Blue
  },
  simonis_760: {
    name: 'Simonis 760',
    material: '70% Wool / 30% Nylon',
    weave: 'Worsted (Napless)',
    baseMuR: 0.012,
    baseMuS: 0.18,
    color: '#0f52ba', // Fast Blue
  },
  league_cloth: {
    name: 'Standard League Cloth',
    material: '75% Wool / 25% Nylon',
    weave: 'Napped',
    baseMuR: 0.022,
    baseMuS: 0.23,
    color: '#0d5c3a', // Classic Green
  },
  old_cloth: {
    name: 'Old Bar Table Cloth',
    material: 'Heavy Napped Wool',
    weave: 'Napped',
    baseMuR: 0.030,
    baseMuS: 0.28,
    color: '#155293',
  },
};

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

    this.aimCanvas = document.getElementById('aim-canvas');
    this.aimCtx = this.aimCanvas ? this.aimCanvas.getContext('2d') : null;

    this.balls = [];
    this.aimAngle = 0; // Radians
    this.cuePower = 2.5; // m/s
    this.cueElevation = 0; // Degrees
    this.spinOffsetX = 0; // [-1.0, 1.0]
    this.spinOffsetY = 0; // [-1.0, 1.0]

    this.isSimulating = false;
    this.isDraggingAim = false;
    this.isBallInHand = false;

    // Cloth Parameters & Environmental State
    this.currentClothKey = 'simonis_860';
    this.humidity = 45; // % RH
    this.temperature = 22; // °C

    // Shot tracking for WPA 9-Ball rules
    this.firstBallHit = null;
    this.railHitAfterContact = false;
    this.pocketedThisShot = [];
    this.targetBallBeforeShot = 1;
    this.consecutiveFouls = [0, 0];
    this.activePlayer = 0;

    this.pockets = [
      new Vector2(BedX, BedY),                              // Top Left
      new Vector2(BedX + BedWidth / 2, BedY - 6),            // Top Center
      new Vector2(BedX + BedWidth, BedY),                   // Top Right
      new Vector2(BedX, BedY + BedHeight),                   // Bottom Left
      new Vector2(BedX + BedWidth / 2, BedY + BedHeight + 6), // Bottom Center
      new Vector2(BedX + BedWidth, BedY + BedHeight),        // Bottom Right
    ];

    this.stats = { shots: 0, pots: 0, fouls: 0 };
    this.eventLogs = [];

    this.initNineBallRack();
    this.bindEvents();
    this.updateClothUI();
    this.startLoop();
  }

  getEffectiveFriction() {
    const preset = ClothPresets[this.currentClothKey] || ClothPresets.simonis_860;
    const humFactorR = 1 + 0.005 * (this.humidity - 45);
    const tempFactorR = 1 - 0.003 * (this.temperature - 22);

    const humFactorS = 1 + 0.003 * (this.humidity - 45);
    const tempFactorS = 1 - 0.002 * (this.temperature - 22);

    return {
      mu_r: Math.max(0.005, preset.baseMuR * humFactorR * tempFactorR),
      mu_s: Math.max(0.05, preset.baseMuS * humFactorS * tempFactorS),
      color: preset.color,
      name: preset.name,
      material: preset.material,
      weave: preset.weave,
    };
  }

  updateClothUI() {
    const eff = this.getEffectiveFriction();
    document.getElementById('cloth-material').innerText = eff.material;
    document.getElementById('cloth-weave').innerText = eff.weave;
    document.getElementById('cloth-mu-r').innerText = eff.mu_r.toFixed(3);
    document.getElementById('cloth-mu-s').innerText = eff.mu_s.toFixed(2);

    const badge = document.getElementById('active-cloth-badge');
    if (badge) {
      badge.innerText = `${eff.name} (${eff.weave.split(' ')[0]})`;
    }
  }

  getLowestRemainingBall() {
    const activeObj = this.balls.filter(b => b.id > 0 && b.active);
    if (activeObj.length === 0) return 9;
    return Math.min(...activeObj.map(b => b.id));
  }

  initNineBallRack() {
    this.balls = [];
    const cueX = BedX + BedWidth * 0.25;
    const cueY = BedY + BedHeight * 0.5;
    this.balls.push(new Ball(0, cueX, cueY));

    const rackX = BedX + BedWidth * 0.72;
    const rackY = BedY + BedHeight * 0.5;
    const r = BallRadiusPX;
    const dx = r * 1.732;

    const positions = [
      { id: 1, x: rackX, y: rackY },
      { id: 2, x: rackX + dx, y: rackY - r },
      { id: 9, x: rackX + dx, y: rackY + r }, // 9 in center
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
    this.targetBallBeforeShot = 1;
    this.consecutiveFouls = [0, 0];
    this.logEvent('WPA 9-Ball rack set on Simonis 860 cloth. Target: Ball #1.', 'info');
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

    cueBall.pos = new Vector2(spawnX, spawnY);
    cueBall.vel = new Vector2(0, 0);
    cueBall.active = true;
    cueBall.state = 'Stationary';

    this.isBallInHand = true;
    document.getElementById('shot-status').innerText = 'Ball in Hand (Click table to place)';
    document.getElementById('shot-status').style.color = '#3b82f6';
    this.logEvent('Cue ball returned to table (Ball in Hand).', 'info');
    this.updateAIDetails();
  }

  spotNineBall() {
    let ball9 = this.balls.find(b => b.id === 9);
    const footSpotX = BedX + BedWidth * 0.72;
    const footSpotY = BedY + BedHeight * 0.5;

    if (!ball9) {
      ball9 = new Ball(9, footSpotX, footSpotY);
      this.balls.push(ball9);
    } else {
      ball9.pos = new Vector2(footSpotX, footSpotY);
      ball9.vel = new Vector2(0, 0);
      ball9.active = true;
      ball9.state = 'Stationary';
    }
    this.logEvent('WPA Rule: 9-Ball spotted on foot spot.', 'info');
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

    // Settings Modal Triggers
    const modal = document.getElementById('settings-modal');
    const openBtn = document.getElementById('btn-open-settings');
    const closeBtn = document.getElementById('btn-close-settings');
    const closeBtnX = document.getElementById('btn-close-settings-x');

    if (openBtn && modal) {
      openBtn.addEventListener('click', () => modal.classList.remove('hidden'));
    }
    if (closeBtn && modal) {
      closeBtn.addEventListener('click', () => modal.classList.add('hidden'));
    }
    if (closeBtnX && modal) {
      closeBtnX.addEventListener('click', () => modal.classList.add('hidden'));
    }

    // Cloth Selectors & Sliders
    const clothSelect = document.getElementById('cloth-preset-select');
    clothSelect.addEventListener('change', (e) => {
      this.currentClothKey = e.target.value;
      this.updateClothUI();
      this.logEvent(`Cloth changed to ${ClothPresets[this.currentClothKey].name}`, 'info');
    });

    const humiditySlider = document.getElementById('humidity-slider');
    humiditySlider.addEventListener('input', (e) => {
      this.humidity = parseInt(e.target.value);
      document.getElementById('humidity-val').innerText = this.humidity;
      this.updateClothUI();
    });

    const tempSlider = document.getElementById('temp-slider');
    tempSlider.addEventListener('input', (e) => {
      this.temperature = parseInt(e.target.value);
      document.getElementById('temp-val').innerText = this.temperature;
      this.updateClothUI();
    });

    document.getElementById('btn-place-cue').addEventListener('click', () => {
      this.respawnCueBall();
    });

    document.getElementById('btn-strike').addEventListener('click', () => {
      this.fireCueStrike();
    });

    document.getElementById('btn-reset-rack').addEventListener('click', () => {
      this.initNineBallRack();
    });

    document.getElementById('btn-ai-hint').addEventListener('click', () => {
      this.applyAIRecommendation();
    });

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

        this.updateAIDetails();
        this.drawAimGraph();
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

      this.updateAIDetails();
      this.drawAimGraph();
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
    this.firstBallHit = null;
    this.railHitAfterContact = false;
    this.pocketedThisShot = [];
    this.targetBallBeforeShot = this.getLowestRemainingBall();

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

    this.logEvent(`Cue struck (Power: ${this.cuePower.toFixed(2)} m/s, Target: Ball #${this.targetBallBeforeShot})`, 'info');
  }

  applyAIRecommendation() {
    let cueBall = this.balls.find(b => b.id === 0);
    if (!cueBall || !cueBall.active) {
      this.respawnCueBall();
      cueBall = this.balls.find(b => b.id === 0);
    }

    const lowestId = this.getLowestRemainingBall();
    const target = this.balls.find(b => b.id === lowestId && b.active);
    if (!cueBall || !target) return;

    const targetPos = target.pos;
    const pocket = this.pockets[2]; // Top Right corner

    const ghostX = targetPos.x - (pocket.x - targetPos.x) / targetPos.dist(pocket) * (2 * BallRadiusPX);
    const ghostY = targetPos.y - (pocket.y - targetPos.y) / targetPos.dist(pocket) * (2 * BallRadiusPX);

    this.aimAngle = Math.atan2(ghostY - cueBall.pos.y, ghostX - cueBall.pos.x);
    this.logEvent(`AI Coach targeted lowest active Ball #${lowestId}.`, 'info');
    this.updateAIDetails();
  }

  getCutAngleAndHitFraction(targetPos, pocketPos) {
    const pocketDir = pocketPos.sub(targetPos).normalize();
    const aimDir = new Vector2(Math.cos(this.aimAngle), Math.sin(this.aimAngle));

    const dot = aimDir.dot(pocketDir);
    const rawAngleRad = Math.acos(Math.max(-1.0, Math.min(1.0, dot)));
    let deg = (rawAngleRad * 180 / Math.PI) % 180;
    if (deg > 90) {
      deg = 180 - deg;
    }
    const cutAngleDeg = Math.max(0, Math.min(90, deg));

    let fractionLabel = 'Full Ball (1/1)';
    if (cutAngleDeg < 5) fractionLabel = 'Full Ball (1/1)';
    else if (cutAngleDeg < 22) fractionLabel = '3/4 Ball';
    else if (cutAngleDeg < 39) fractionLabel = '1/2 Ball';
    else if (cutAngleDeg < 55) fractionLabel = '1/4 Ball';
    else if (cutAngleDeg < 72) fractionLabel = '1/8 Ball';
    else fractionLabel = 'Thin Glance';

    return {
      angleDeg: cutAngleDeg,
      fractionLabel: fractionLabel,
      displayString: `${cutAngleDeg.toFixed(1)}° (${fractionLabel})`,
    };
  }

  updateAIDetails() {
    const cueBall = this.balls.find(b => b.id === 0);
    const lowestId = this.getLowestRemainingBall();
    const target = this.balls.find(b => b.id === lowestId && b.active);

    if (cueBall && target) {
      const pocket = this.pockets[2];
      const cutInfo = this.getCutAngleAndHitFraction(target.pos, pocket);

      document.getElementById('ai-target-ball').innerText = `Ball #${lowestId}`;
      document.getElementById('ai-target-pocket').innerText = 'Corner Right';
      document.getElementById('ai-cut-angle').innerText = cutInfo.displayString;

      const cutBadge = document.getElementById('cut-angle-badge');
      if (cutBadge) {
        cutBadge.innerText = `${cutInfo.angleDeg.toFixed(1)}° • ${cutInfo.fractionLabel}`;
      }
    }
  }

  evaluateShotRules() {
    const lowestRequired = this.targetBallBeforeShot || this.getLowestRemainingBall();
    let fouls = [];

    const cueBall = this.balls.find(b => b.id === 0);
    if (!cueBall || !cueBall.active) {
      fouls.push('Scratch');
    }

    if (!this.firstBallHit) {
      fouls.push('No Contact');
    } else if (this.firstBallHit.id !== lowestRequired) {
      fouls.push(`Wrong Ball First (Hit #${this.firstBallHit.id}, Required #${lowestRequired})`);
    }

    if (this.firstBallHit && this.pocketedThisShot.length === 0 && !this.railHitAfterContact) {
      fouls.push('No Rail Contact After Impact');
    }

    if (fouls.length > 0) {
      this.stats.fouls += 1;
      document.getElementById('stat-fouls').innerText = this.stats.fouls;
      this.logEvent(`FOUL COMMITTED: ${fouls.join(' | ')} (Ball in Hand given)`, 'foul');

      if (this.pocketedThisShot.includes(9)) {
        this.spotNineBall();
      }

      this.respawnCueBall();
    } else {
      // Legal Shot
      document.getElementById('shot-status').innerText = 'Ready for Shot';
      document.getElementById('shot-status').style.color = '#10b981';

      if (this.pocketedThisShot.includes(9)) {
        this.logEvent('VICTORY! Legally pocketed the 9-Ball!', 'pocket');
        alert('🎉 VICTORY! Legally pocketed the 9-Ball!');
      } else if (this.pocketedThisShot.length > 0) {
        this.logEvent(`Legal Shot: Pocketed ${this.pocketedThisShot.length} ball(s). Continue shooting.`, 'pocket');
      } else {
        this.logEvent('Legal Safety / Position shot completed.', 'info');
      }
      this.updateAIDetails();
    }
  }

  stepPhysics(dt) {
    const maxSubStepDt = 0.001; // 1 ms sub-step integration (1000 Hz) to eliminate tunneling at high strike power
    let remainingDt = dt;

    while (remainingDt > 0) {
      const subDt = Math.min(remainingDt, maxSubStepDt);
      this.internalSubStep(subDt);
      remainingDt -= subDt;
    }

    const anyMoving = this.balls.some(b => b.isMoving());
    if (!anyMoving && this.isSimulating) {
      this.isSimulating = false;
      this.evaluateShotRules();
    }
  }

  internalSubStep(dt) {
    const eff = this.getEffectiveFriction();
    const g = Gravity;
    const mu_s = eff.mu_s;
    const mu_r = eff.mu_r;

    // 1. Pairwise Collisions
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

            if (!this.firstBallHit) {
              if (b1.id === 0) this.firstBallHit = b2;
              else if (b2.id === 0) this.firstBallHit = b1;
            }

            const overlap = (minDist - dist) / 2;
            b1.pos = b1.pos.sub(normal.mul(overlap));
            b2.pos = b2.pos.add(normal.mul(overlap));

            this.logEvent(`Collision: Ball #${b1.id} <-> Ball #${b2.id}`, 'collision');
          }
        }
      }
    }

    // 2. Rail & Pocket Captures
    for (const b of this.balls) {
      if (!b.active) continue;

      for (let pIdx = 0; pIdx < this.pockets.length; pIdx++) {
        if (b.pos.dist(this.pockets[pIdx]) < PocketRadiusPX) {
          b.active = false;
          b.state = 'Pocketed';
          b.vel = new Vector2(0, 0);

          if (b.id !== 0) {
            this.pocketedThisShot.push(b.id);
            this.stats.pots += 1;
            document.getElementById('stat-pots').innerText = this.stats.pots;
            this.logEvent(`Ball #${b.id} pocketed!`, 'pocket');
          }
          break;
        }
      }

      if (!b.active) continue;

      const left = BedX + BallRadiusPX;
      const right = BedX + BedWidth - BallRadiusPX;
      const top = BedY + BallRadiusPX;
      const bottom = BedY + BedHeight - BallRadiusPX;

      let hitRail = false;
      if (b.pos.x < left) { b.pos.x = left; b.vel.x = -b.vel.x * RestitutionRail; hitRail = true; }
      if (b.pos.x > right) { b.pos.x = right; b.vel.x = -b.vel.x * RestitutionRail; hitRail = true; }
      if (b.pos.y < top) { b.pos.y = top; b.vel.y = -b.vel.y * RestitutionRail; hitRail = true; }
      if (b.pos.y > bottom) { b.pos.y = bottom; b.vel.y = -b.vel.y * RestitutionRail; hitRail = true; }

      if (hitRail && this.firstBallHit) {
        this.railHitAfterContact = true;
      }

      const speed = b.vel.len();
      if (speed > 5) {
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
  }

  logEvent(msg, type) {
    const container = document.getElementById('event-log');
    const item = document.createElement('div');
    item.className = `log-item ${type}`;
    item.innerText = `[${new Date().toLocaleTimeString()}] ${msg}`;
    container.prepend(item);
  }

  drawAimGraph() {
    if (!this.aimCtx) return;
    const ctx = this.aimCtx;
    const w = this.aimCanvas.width;
    const h = this.aimCanvas.height;

    ctx.clearRect(0, 0, w, h);

    // Dark Background
    ctx.fillStyle = '#0f172a';
    ctx.fillRect(0, 0, w, h);

    // Subtle Grid
    ctx.strokeStyle = 'rgba(255, 255, 255, 0.05)';
    ctx.lineWidth = 1;
    for (let x = 0; x < w; x += 20) {
      ctx.beginPath(); ctx.moveTo(x, 0); ctx.lineTo(x, h); ctx.stroke();
    }
    for (let y = 0; y < h; y += 20) {
      ctx.beginPath(); ctx.moveTo(0, y); ctx.lineTo(w, y); ctx.stroke();
    }

    const cueBall = this.balls.find(b => b.id === 0);
    const lowestId = this.getLowestRemainingBall();
    const target = this.balls.find(b => b.id === lowestId && b.active);

    const r = 14; // Radius for HD diagram rendering
    const cuePos = new Vector2(45, h / 2 + 10);
    const ghostPos = new Vector2(160, h / 2 + 10);

    const aimDir = new Vector2(Math.cos(this.aimAngle), Math.sin(this.aimAngle));
    const aimNorm = new Vector2(-aimDir.y, aimDir.x);

    // Compute Cut Angle & Pool Hit Fraction
    const pocket = this.pockets[2];
    let cutInfo = { angleDeg: 24.5, fractionLabel: '1/2 Ball' };
    let targetPos = new Vector2(ghostPos.x + 2 * r, ghostPos.y);

    if (cueBall && target) {
      cutInfo = this.getCutAngleAndHitFraction(target.pos, pocket);
      const angleRad = cutInfo.angleDeg * Math.PI / 180;
      targetPos = ghostPos.add(new Vector2(Math.cos(angleRad) * 2 * r, -Math.sin(angleRad) * 2 * r));
    }

    // 1. Draw Line of Aim (Cue Ball -> Ghost Ball)
    ctx.strokeStyle = '#38bdf8';
    ctx.lineWidth = 2;
    ctx.setLineDash([4, 4]);
    ctx.beginPath();
    ctx.moveTo(cuePos.x, cuePos.y);
    ctx.lineTo(ghostPos.x, ghostPos.y);
    ctx.stroke();
    ctx.setLineDash([]);

    // 2. Draw Target Line (Ghost Ball / Target Ball -> Pocket Direction)
    ctx.strokeStyle = '#10b981';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.moveTo(targetPos.x, targetPos.y);
    ctx.lineTo(targetPos.x + 50, targetPos.y - 20);
    ctx.stroke();

    // 3. Draw Cut Angle Arc & Fraction Label
    ctx.strokeStyle = '#f59e0b';
    ctx.lineWidth = 2;
    ctx.beginPath();
    ctx.arc(ghostPos.x, ghostPos.y, 28, 0, -cutInfo.angleDeg * Math.PI / 180, true);
    ctx.stroke();

    ctx.fillStyle = '#fbbf24';
    ctx.font = 'bold 10px JetBrains Mono, monospace';
    ctx.textAlign = 'center';
    ctx.fillText(`${cutInfo.angleDeg.toFixed(1)}° (${cutInfo.fractionLabel})`, ghostPos.x + 35, ghostPos.y - 14);

    // 4. Draw Cue Ball (White) & Cue Tip Contact Point
    ctx.fillStyle = '#ffffff';
    ctx.beginPath();
    ctx.arc(cuePos.x, cuePos.y, r, 0, Math.PI * 2);
    ctx.fill();
    ctx.strokeStyle = '#64748b';
    ctx.lineWidth = 2;
    ctx.stroke();

    // Cue Tip Contact Offset Marker & Vertical Dotted Line Indicator
    const tipX = cuePos.x + this.spinOffsetX * (r * 0.70);
    const tipY = cuePos.y - this.spinOffsetY * (r * 0.70);

    // Vertical Dotted Line through Cue Tip Contact Point
    ctx.strokeStyle = 'rgba(239, 68, 68, 0.85)';
    ctx.lineWidth = 1.5;
    ctx.setLineDash([2, 3]);
    ctx.beginPath();
    ctx.moveTo(tipX, cuePos.y - r - 6);
    ctx.lineTo(tipX, cuePos.y + r + 6);
    ctx.stroke();
    ctx.setLineDash([]);

    // Cue Tip Crosshair Dot
    ctx.fillStyle = '#ef4444';
    ctx.beginPath();
    ctx.arc(tipX, tipY, 3.0, 0, Math.PI * 2);
    ctx.fill();
    ctx.strokeStyle = '#ffffff';
    ctx.lineWidth = 1;
    ctx.stroke();

    // 5. Draw Ghost Ball (Dashed Outline)
    ctx.strokeStyle = '#38bdf8';
    ctx.lineWidth = 2;
    ctx.setLineDash([4, 4]);
    ctx.beginPath();
    ctx.arc(ghostPos.x, ghostPos.y, r, 0, Math.PI * 2);
    ctx.stroke();
    ctx.setLineDash([]);

    // 6. Draw Target Object Ball (Colored with Ball ID)
    const targetColor = BallColors[lowestId] || '#facc15';
    ctx.fillStyle = targetColor;
    ctx.beginPath();
    ctx.arc(targetPos.x, targetPos.y, r, 0, Math.PI * 2);
    ctx.fill();
    ctx.strokeStyle = '#0f172a';
    ctx.lineWidth = 1.5;
    ctx.stroke();

    ctx.fillStyle = '#ffffff';
    ctx.beginPath();
    ctx.arc(targetPos.x, targetPos.y, r * 0.45, 0, Math.PI * 2);
    ctx.fill();
    ctx.fillStyle = '#1e293b';
    ctx.font = 'bold 9px Inter, sans-serif';
    ctx.textAlign = 'center';
    ctx.textBaseline = 'middle';
    ctx.fillText(lowestId.toString(), targetPos.x, targetPos.y + 1);

    // 7. Impact Contact Point & Vertical Dotted Line Indicator on Target
    const contactPoint = ghostPos.add(targetPos.sub(ghostPos).normalize().mul(r));

    // Vertical Dotted Line Indicator passing through impact contact point across target view
    ctx.strokeStyle = 'rgba(239, 68, 68, 0.85)';
    ctx.lineWidth = 1.5;
    ctx.setLineDash([3, 3]);
    ctx.beginPath();
    ctx.moveTo(contactPoint.x, 12);
    ctx.lineTo(contactPoint.x, h - 12);
    ctx.stroke();
    ctx.setLineDash([]);

    // Impact Contact Point Accent Dot
    ctx.fillStyle = '#ef4444';
    ctx.beginPath();
    ctx.arc(contactPoint.x, contactPoint.y, 4.0, 0, Math.PI * 2);
    ctx.fill();
    ctx.strokeStyle = '#ffffff';
    ctx.lineWidth = 1.5;
    ctx.stroke();
  }

  draw() {
    const ctx = this.ctx;
    const eff = this.getEffectiveFriction();

    ctx.clearRect(0, 0, CanvasWidth, CanvasHeight);

    // Wooden Rail Frame
    ctx.fillStyle = '#2d1810';
    ctx.fillRect(0, 0, CanvasWidth, CanvasHeight);

    // Slate Cloth Bed (Simonis 860 Tournament Blue / Selected Cloth Color)
    ctx.fillStyle = eff.color;
    ctx.fillRect(BedX, BedY, BedWidth, BedHeight);

    // Cushion Borders
    ctx.strokeStyle = '#022c22';
    ctx.lineWidth = 4;
    ctx.strokeRect(BedX, BedY, BedWidth, BedHeight);

    // Diamonds / Sights along rails
    ctx.fillStyle = '#f8fafc';
    for (let i = 1; i <= 3; i++) {
      ctx.beginPath(); ctx.arc(BedX + (BedWidth / 4) * i, BedY / 2, 3, 0, Math.PI * 2); ctx.fill();
      ctx.beginPath(); ctx.arc(BedX + (BedWidth / 4) * i, CanvasHeight - BedY / 2, 3, 0, Math.PI * 2); ctx.fill();
    }

    for (const pocket of this.pockets) {
      ctx.fillStyle = '#111827';
      ctx.beginPath();
      ctx.arc(pocket.x, pocket.y, PocketRadiusPX, 0, Math.PI * 2);
      ctx.fill();
      ctx.strokeStyle = '#374151';
      ctx.lineWidth = 3;
      ctx.stroke();
    }

    const cueBall = this.balls.find(b => b.id === 0);
    if (cueBall && cueBall.active && !this.isSimulating) {
      const aimDirX = Math.cos(this.aimAngle);
      const aimDirY = Math.sin(this.aimAngle);

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

      ctx.strokeStyle = 'rgba(255, 255, 255, 0.4)';
      ctx.lineWidth = 2;
      ctx.setLineDash([6, 6]);
      ctx.beginPath();
      ctx.moveTo(cueBall.pos.x, cueBall.pos.y);
      ctx.lineTo(ghostX, ghostY);
      ctx.stroke();
      ctx.setLineDash([]);

      ctx.strokeStyle = 'rgba(255, 255, 255, 0.8)';
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.arc(ghostX, ghostY, BallRadiusPX, 0, Math.PI * 2);
      ctx.stroke();

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

    for (const ball of this.balls) {
      if (!ball.active) continue;

      ctx.fillStyle = 'rgba(0, 0, 0, 0.35)';
      ctx.beginPath();
      ctx.arc(ball.pos.x + 3, ball.pos.y + 4, BallRadiusPX, 0, Math.PI * 2);
      ctx.fill();

      ctx.fillStyle = BallColors[ball.id] || '#fff';
      ctx.beginPath();
      ctx.arc(ball.pos.x, ball.pos.y, BallRadiusPX, 0, Math.PI * 2);
      ctx.fill();

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

    // Render HD Aiming Diagram
    this.drawAimGraph();
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

window.addEventListener('DOMContentLoaded', () => {
  const canvas = document.getElementById('table-canvas');
  window.cueForgeApp = new CueForgeSimulation(canvas);
});
