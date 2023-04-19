use crate::scene::vertex::Vertex;
use cgmath::{perspective, vec3, vec4, Matrix4, Vector3, Vector4};
use sdl2::rect::Point;
use std::ops::Mul;

pub struct Camera {
    viewport_height: f64,
    viewport_width: f64,
    global_state_vector: Vector4<f64>,
    rotation_x: f64,
    rotation_matrix_x: Matrix4<f64>,
    rotation_y: f64,
    rotation_matrix_y: Matrix4<f64>,
    projection_plane_distance: f64,
    projection_matrix: Matrix4<f64>,
    rotation_transform_matrix: Matrix4<f64>,
}

impl Camera {
    pub fn new(
        viewport_height: f64,
        viewport_width: f64,
        projection_plane_distance: f64,
        global_state_vector: Vector4<f64>,
    ) -> Camera {
        let startig_rotation_x = 0.0;
        let startig_rotation_y = 0.0;

        Camera {
            viewport_height,
            viewport_width,
            projection_plane_distance,
            projection_matrix: perspective(
                cgmath::Deg(80.0),
                viewport_width / viewport_height,
                0.1,
                100.0,
            ),
            global_state_vector,
            rotation_matrix_x: Matrix4::from_axis_angle(
                vec3(1.0, 0.0, 0.0),
                cgmath::Deg(startig_rotation_x),
            ),
            rotation_matrix_y: Matrix4::from_axis_angle(
                vec3(0.0, 1.0, 0.0),
                cgmath::Deg(-startig_rotation_y),
            ),
            rotation_x: startig_rotation_x,
            rotation_y: startig_rotation_y,
            rotation_transform_matrix: Matrix4::from_axis_angle(
                vec3(0.0, 1.0, 0.0),
                cgmath::Deg(startig_rotation_y),
            ),
        }
    }

    pub fn project(&self, vertex: Vertex) -> Result<Point, String> {
        let rotated_global_state_vector = self.rotation_matrix_y.mul(self.global_state_vector);
        let vertex_in_quadrilateral = self.convert_vertex_to_quadrilateral(vertex);
        let point_rotated_around_y = self.rotation_matrix_y.mul(vertex_in_quadrilateral);
        let projected_point = self.projection_matrix.mul(
            self.rotation_matrix_x
                .mul(point_rotated_around_y + rotated_global_state_vector),
        );

        let clipped_vertex = clip_quadrilateral(projected_point);

        // TODO... make it better xd
        if Camera::is_not_visible(clipped_vertex) {
            return Err("Vertex is not visible".to_string());
        }

        let projected_x = (clipped_vertex.x * self.viewport_width)
            / (self.projection_plane_distance * clipped_vertex.z)
            + self.viewport_width / 2.;
        let projected_y = (clipped_vertex.y * self.viewport_height)
            / (self.projection_plane_distance * clipped_vertex.z)
            + self.viewport_height / 2.;

        Ok(Point::new(projected_x as i32, projected_y as i32))
    }

    fn convert_vertex_to_quadrilateral(&self, vertex: Vertex) -> Vector4<f64> {
        return vec4(vertex.x, vertex.y, vertex.z, 1.);
    }

    fn is_not_visible(vertex: Vector3<f64>) -> bool {
        return vertex.z < 1.;
    }

    pub fn rotate_x(&mut self, angle: f64) {
        self.rotation_x += angle;
        self.rotation_matrix_x =
            Matrix4::from_axis_angle(vec3(1.0, 0.0, 0.0), cgmath::Deg(self.rotation_x));
    }

    pub fn rotate_y(&mut self, angle: f64) {
        self.rotation_y += angle;
        self.rotation_matrix_y =
            Matrix4::from_axis_angle(vec3(0.0, 1.0, 0.0), cgmath::Deg(-self.rotation_y));
    }

    pub fn translate_forward(&mut self) {
        self.rotation_transform_matrix =
            Matrix4::from_axis_angle(vec3(0.0, 1.0, 0.0), cgmath::Deg(self.rotation_y));
        self.global_state_vector -= self.rotation_transform_matrix.mul(vec4(0.0, 0.0, 1., 0.0));
    }

    pub fn translate_backward(&mut self) {
        self.rotation_transform_matrix =
            Matrix4::from_axis_angle(vec3(0.0, 1.0, 0.0), cgmath::Deg(self.rotation_y));
        self.global_state_vector += self.rotation_transform_matrix.mul(vec4(0.0, 0.0, 1., 0.0));
    }

    pub fn translate_left(&mut self) {
        self.global_state_vector -= self.rotation_transform_matrix.mul(vec4(1.0, 0.0, 0., 0.0));
    }

    pub fn translate_right(&mut self) {
        self.global_state_vector += self.rotation_transform_matrix.mul(vec4(1.0, 0.0, 0., 0.0));
    }

    pub fn change_zoom(&mut self, zoom_change: f64) {
        self.projection_plane_distance += zoom_change;
        println!("The zoom is {}", self.projection_plane_distance);
    }

    pub fn reset_zoom(&mut self) {
        self.projection_plane_distance = 1.
    }
}

fn clip_quadrilateral(quadrilateral: Vector4<f64>) -> Vector3<f64> {
    return vec3(
        quadrilateral.x / quadrilateral.w,
        quadrilateral.y / quadrilateral.w,
        quadrilateral.z / quadrilateral.w,
    );
}
