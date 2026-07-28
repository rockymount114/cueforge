//! `cueforge-vision`
//!
//! Computer vision detection types, 4-point table homography calibration, and tracking pipeline.

use cueforge_common::Vec2;

/// Ball detection result from physical camera tracker.
#[derive(Debug, Clone, PartialEq)]
pub struct DetectedBall {
    pub ball_id: u32,
    pub camera_pixel: (f64, f64),
    pub table_position: Vec2,
    pub confidence: f64,
}

/// 4-Point Homography calibrator mapping camera pixel space (u, v) to table physical coordinates (x, y).
#[derive(Debug, Clone, PartialEq)]
pub struct TableHomography {
    /// 4 camera corner pixels: [top-left, top-right, bottom-right, bottom-left]
    pub src_corners: [(f64, f64); 4],
    /// 4 table physical corner coordinates
    pub dst_corners: [Vec2; 4],
}

impl TableHomography {
    pub fn new(table_width: f64, table_length: f64, src_corners: [(f64, f64); 4]) -> Self {
        let half_w = table_width / 2.0;
        let half_l = table_length / 2.0;

        let dst_corners = [
            Vec2::new(-half_w, half_l),  // Top-Left
            Vec2::new(half_w, half_l),   // Top-Right
            Vec2::new(half_w, -half_l),  // Bottom-Right
            Vec2::new(-half_w, -half_l), // Bottom-Left
        ];

        Self {
            src_corners,
            dst_corners,
        }
    }

    /// Map a camera pixel coordinate (u, v) into physical table plane coordinate (x, y) using bilinear interpolation.
    pub fn map_pixel_to_table(&self, u: f64, v: f64) -> Vec2 {
        let (u0, v0) = self.src_corners[0];
        let (u1, v1) = self.src_corners[2];

        let norm_u = ((u - u0) / (u1 - u0).max(1e-5)).clamp(0.0, 1.0);
        let norm_v = ((v - v0) / (v1 - v0).max(1e-5)).clamp(0.0, 1.0);

        let tl = self.dst_corners[0];
        let tr = self.dst_corners[1];
        let br = self.dst_corners[2];
        let bl = self.dst_corners[3];

        let top = tl + (tr - tl) * norm_u;
        let bottom = bl + (br - bl) * norm_u;

        top + (bottom - top) * norm_v
    }
}

/// Vision tracking pipeline filtering raw camera detections.
#[derive(Debug, Clone)]
pub struct VisionPipeline {
    pub homography: TableHomography,
    pub confidence_threshold: f64,
}

impl VisionPipeline {
    pub fn new(homography: TableHomography, confidence_threshold: f64) -> Self {
        Self {
            homography,
            confidence_threshold,
        }
    }

    pub fn process_detection(
        &self,
        ball_id: u32,
        pixel_u: f64,
        pixel_v: f64,
        confidence: f64,
    ) -> Option<DetectedBall> {
        if confidence < self.confidence_threshold {
            return None;
        }

        let table_pos = self.homography.map_pixel_to_table(pixel_u, pixel_v);

        Some(DetectedBall {
            ball_id,
            camera_pixel: (pixel_u, pixel_v),
            table_position: table_pos,
            confidence,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_homography_mapping() {
        let corners = [(0.0, 0.0), (1000.0, 0.0), (1000.0, 2000.0), (0.0, 2000.0)];
        let homography = TableHomography::new(1.27, 2.54, corners);
        let pipeline = VisionPipeline::new(homography, 0.75);

        let detection = pipeline.process_detection(1, 500.0, 1000.0, 0.90);
        assert!(detection.is_some());

        let det = detection.unwrap();
        assert!((det.table_position.x - 0.0).abs() < 1e-3);
        assert!((det.table_position.y - 0.0).abs() < 1e-3);
    }
}
