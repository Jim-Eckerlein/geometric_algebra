struct Scalar {
    // 1
    float g0;
};

struct MultiVector {
    // 1, e23, -e13, e12
    vec4 g0;
    // e0, -e023, e013, -e012
    vec4 g1;
    // e123, e1, e2, e3
    vec4 g2;
    // e0123, e01, e02, e03
    vec4 g3;
};

struct Rotor {
    // 1, e23, -e13, e12
    vec4 g0;
};

struct Point {
    // e123, -e023, e013, -e012
    vec4 g0;
};

struct Plane {
    // e0, e1, e2, e3
    vec4 g0;
};

struct Line {
    // e01, e02, e03
    vec3 g0;
    // e23, -e13, e12
    vec3 g1;
};

struct Translator {
    // 1, e01, e02, e03
    vec4 g0;
};

struct Motor {
    // 1, e23, -e13, e12
    vec4 g0;
    // e0123, e01, e02, e03
    vec4 g1;
};

struct Branch {
    // e01, e02, e03
    vec3 g0;
};

Scalar scalar_zero() {
    return Scalar(0.0);
}

Scalar scalar_one() {
    return Scalar(1.0);
}

Scalar scalar_neg(Scalar self) {
    return Scalar(self.g0 * -1.0);
}

Scalar scalar_automorphism(Scalar self) {
    return Scalar(self.g0);
}

Scalar scalar_reversal(Scalar self) {
    return Scalar(self.g0);
}

Scalar scalar_conjugation(Scalar self) {
    return Scalar(self.g0);
}

Scalar scalar_scalar_add(Scalar self, Scalar other) {
    return Scalar(self.g0 + other.g0);
}

Scalar scalar_scalar_sub(Scalar self, Scalar other) {
    return Scalar(self.g0 - other.g0);
}

Scalar scalar_scalar_geometric_product(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_outer_product(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_inner_product(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_left_contraction(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_right_contraction(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

Scalar scalar_scalar_scalar_product(Scalar self, Scalar other) {
    return Scalar(self.g0 * other.g0);
}

MultiVector scalar_multi_vector_add(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0, other.g1, other.g2, other.g3);
}

MultiVector scalar_multi_vector_sub(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0, vec4(0.0) - other.g1, vec4(0.0) - other.g2, vec4(0.0) - other.g3);
}

MultiVector scalar_multi_vector_geometric_product(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1, vec4(self.g0) * other.g2, vec4(self.g0) * other.g3);
}

Scalar scalar_multi_vector_regressive_product(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g3.x);
}

MultiVector scalar_multi_vector_outer_product(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1, vec4(self.g0) * other.g2, vec4(self.g0) * other.g3);
}

MultiVector scalar_multi_vector_inner_product(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1, vec4(self.g0) * other.g2, vec4(self.g0) * other.g3);
}

MultiVector scalar_multi_vector_left_contraction(Scalar self, MultiVector other) {
    return MultiVector(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1, vec4(self.g0) * other.g2, vec4(self.g0) * other.g3);
}

Scalar scalar_multi_vector_right_contraction(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_multi_vector_scalar_product(Scalar self, MultiVector other) {
    return Scalar(self.g0 * other.g0.x);
}

Rotor scalar_rotor_add(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0);
}

Rotor scalar_rotor_sub(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0);
}

Rotor scalar_rotor_geometric_product(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor scalar_rotor_outer_product(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor scalar_rotor_inner_product(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Rotor scalar_rotor_left_contraction(Scalar self, Rotor other) {
    return Rotor(vec4(self.g0) * other.g0);
}

Scalar scalar_rotor_right_contraction(Scalar self, Rotor other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_rotor_scalar_product(Scalar self, Rotor other) {
    return Scalar(self.g0 * other.g0.x);
}

Point scalar_point_geometric_product(Scalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Point scalar_point_outer_product(Scalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Point scalar_point_inner_product(Scalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Point scalar_point_left_contraction(Scalar self, Point other) {
    return Point(vec4(self.g0) * other.g0);
}

Plane scalar_plane_geometric_product(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane scalar_plane_outer_product(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane scalar_plane_inner_product(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Plane scalar_plane_left_contraction(Scalar self, Plane other) {
    return Plane(vec4(self.g0) * other.g0);
}

Line scalar_line_geometric_product(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line scalar_line_outer_product(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line scalar_line_inner_product(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Line scalar_line_left_contraction(Scalar self, Line other) {
    return Line(vec3(self.g0) * other.g0, vec3(self.g0) * other.g1);
}

Translator scalar_translator_add(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0);
}

Translator scalar_translator_sub(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0);
}

Translator scalar_translator_geometric_product(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator scalar_translator_outer_product(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator scalar_translator_inner_product(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Translator scalar_translator_left_contraction(Scalar self, Translator other) {
    return Translator(vec4(self.g0) * other.g0);
}

Scalar scalar_translator_right_contraction(Scalar self, Translator other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_translator_scalar_product(Scalar self, Translator other) {
    return Scalar(self.g0 * other.g0.x);
}

Motor scalar_motor_add(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0, other.g1);
}

Motor scalar_motor_sub(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0, vec4(0.0) - other.g1);
}

Motor scalar_motor_geometric_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Scalar scalar_motor_regressive_product(Scalar self, Motor other) {
    return Scalar(self.g0 * other.g1.x);
}

Motor scalar_motor_outer_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Motor scalar_motor_inner_product(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Motor scalar_motor_left_contraction(Scalar self, Motor other) {
    return Motor(vec4(self.g0) * other.g0, vec4(self.g0) * other.g1);
}

Scalar scalar_motor_right_contraction(Scalar self, Motor other) {
    return Scalar(self.g0 * other.g0.x);
}

Scalar scalar_motor_scalar_product(Scalar self, Motor other) {
    return Scalar(self.g0 * other.g0.x);
}

Translator scalar_branch_add(Scalar self, Branch other) {
    return Translator(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Translator scalar_branch_sub(Scalar self, Branch other) {
    return Translator(vec4(self.g0) * vec4(1.0, 0.0, 0.0, 0.0) - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Branch scalar_branch_geometric_product(Scalar self, Branch other) {
    return Branch(vec3(self.g0) * other.g0);
}

Branch scalar_branch_outer_product(Scalar self, Branch other) {
    return Branch(vec3(self.g0) * other.g0);
}

Branch scalar_branch_inner_product(Scalar self, Branch other) {
    return Branch(vec3(self.g0) * other.g0);
}

Branch scalar_branch_left_contraction(Scalar self, Branch other) {
    return Branch(vec3(self.g0) * other.g0);
}

Scalar scalar_squared_magnitude(Scalar self) {
    return scalar_scalar_scalar_product(self, scalar_reversal(self));
}

Scalar scalar_magnitude(Scalar self) {
    return Scalar(sqrt(scalar_squared_magnitude(self).g0));
}

Scalar scalar_signum(Scalar self) {
    return scalar_scalar_geometric_product(self, Scalar(1.0 / scalar_magnitude(self).g0));
}

Scalar scalar_inverse(Scalar self) {
    return scalar_scalar_geometric_product(scalar_reversal(self), Scalar(1.0 / scalar_squared_magnitude(self).g0));
}

MultiVector multi_vector_zero() {
    return MultiVector(vec4(0.0), vec4(0.0), vec4(0.0), vec4(0.0));
}

MultiVector multi_vector_one() {
    return MultiVector(vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0), vec4(0.0), vec4(0.0));
}

MultiVector multi_vector_neg(MultiVector self) {
    return MultiVector(self.g0 * vec4(-1.0), self.g1 * vec4(-1.0), self.g2 * vec4(-1.0), self.g3 * vec4(-1.0));
}

MultiVector multi_vector_automorphism(MultiVector self) {
    return MultiVector(self.g0, self.g1 * vec4(-1.0), self.g2 * vec4(-1.0), self.g3);
}

MultiVector multi_vector_reversal(MultiVector self) {
    return MultiVector(self.g0 * vec4(1.0, -1.0, -1.0, -1.0), self.g1 * vec4(1.0, -1.0, -1.0, -1.0), self.g2 * vec4(-1.0, 1.0, 1.0, 1.0), self.g3 * vec4(1.0, -1.0, -1.0, -1.0));
}

MultiVector multi_vector_conjugation(MultiVector self) {
    return MultiVector(self.g0 * vec4(1.0, -1.0, -1.0, -1.0), self.g1 * vec4(-1.0, 1.0, 1.0, 1.0), self.g2 * vec4(1.0, -1.0, -1.0, -1.0), self.g3 * vec4(1.0, -1.0, -1.0, -1.0));
}

MultiVector multi_vector_dual(MultiVector self) {
    return MultiVector(self.g3, self.g2 * vec4(-1.0, 1.0, 1.0, 1.0), self.g1 * vec4(1.0, -1.0, -1.0, -1.0), self.g0);
}

Scalar multi_vector_scalar_into(MultiVector self) {
    return Scalar(self.g0.x);
}

MultiVector multi_vector_scalar_add(MultiVector self, Scalar other) {
    return MultiVector(self.g0 + vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0), self.g1, self.g2, self.g3);
}

MultiVector multi_vector_scalar_sub(MultiVector self, Scalar other) {
    return MultiVector(self.g0 - vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0), self.g1, self.g2, self.g3);
}

MultiVector multi_vector_scalar_geometric_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0), self.g2 * vec4(other.g0), self.g3 * vec4(other.g0));
}

Scalar multi_vector_scalar_regressive_product(MultiVector self, Scalar other) {
    return Scalar(self.g3.x * other.g0);
}

MultiVector multi_vector_scalar_outer_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0), self.g2 * vec4(other.g0), self.g3 * vec4(other.g0));
}

MultiVector multi_vector_scalar_inner_product(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0), self.g2 * vec4(other.g0), self.g3 * vec4(other.g0));
}

Scalar multi_vector_scalar_left_contraction(MultiVector self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector multi_vector_scalar_right_contraction(MultiVector self, Scalar other) {
    return MultiVector(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0), self.g2 * vec4(other.g0), self.g3 * vec4(other.g0));
}

Scalar multi_vector_scalar_scalar_product(MultiVector self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector multi_vector_multi_vector_add(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 + other.g0, self.g1 + other.g1, self.g2 + other.g2, self.g3 + other.g3);
}

MultiVector multi_vector_multi_vector_sub(MultiVector self, MultiVector other) {
    return MultiVector(self.g0 - other.g0, self.g1 - other.g1, self.g2 - other.g2, self.g3 - other.g3);
}

MultiVector multi_vector_multi_vector_geometric_product(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g2.x) * other.g2 * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g2.y) * other.g2.yxwz * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g2.z) * other.g2.zwxy * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g2.w) * other.g2.wzyx * vec4(1.0, -1.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g2.x) * other.g3 + vec4(self.g2.y) * other.g3.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g2.z) * other.g3.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g2.w) * other.g3.wzyx * vec4(-1.0, -1.0, 1.0, 1.0) - vec4(self.g3.x) * other.g2 + vec4(self.g3.y) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g3.z) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g3.w) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.y) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g2.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g2.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.y) * other.g3.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g3.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g3.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g2 + vec4(self.g1.y) * other.g2.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g2.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.w) * other.g2.wzyx * vec4(-1.0, -1.0, 1.0, 1.0) - vec4(self.g2.x) * other.g1 + vec4(self.g2.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g2.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g2.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g3.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g3.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

MultiVector multi_vector_multi_vector_regressive_product(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.y) * other.g3.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g3.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g3.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g2.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g2.yxyy * vec4(-1.0, -1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g2.zzxz * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g1.w) * other.g2.wwwx * vec4(-1.0, 0.0, 0.0, -1.0) + vec4(self.g2.x) * other.g1 * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g2.y) * vec4(other.g1.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.x) * other.g0 + vec4(self.g3.y) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g3.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.y) * other.g3.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g3.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g3.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * other.g1 + vec4(self.g3.y) * vec4(other.g1.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g1.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g1.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g3.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.z) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.y) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g2.x) * other.g3 + vec4(self.g2.z) * vec4(other.g3.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * other.g2 + vec4(self.g3.y) * vec4(other.g2.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g2.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x, self.g2.y, self.g0.y, self.g0.y) * vec4(other.g1.x, other.g3.x, other.g1.w, other.g1.z) * vec4(0.0, 1.0, -1.0, 1.0), vec4(self.g1.z) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g3.x) * other.g3 + vec4(self.g3.z) * vec4(other.g3.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g3.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g3.y, self.g1.y, self.g1.y) * vec4(other.g1.x, other.g3.x, other.g1.w, other.g1.z) * vec4(0.0, 1.0, -1.0, 1.0));
}

MultiVector multi_vector_multi_vector_outer_product(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g2.y) * other.g2.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * other.g2.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * other.g2.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g2.y) * other.g3.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * other.g3.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * other.g3.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g3.y) * other.g2.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g3.z) * other.g2.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g3.w) * other.g2.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g2.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.z) * vec4(other.g3.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g3.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * other.g2 + vec4(self.g1.y) * vec4(other.g2.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g2.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g2.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g2.x) * vec4(other.g1.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g2.y) * other.g1.yxyy * vec4(1.0, -1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g1.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g2.w) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g3.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g3.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector multi_vector_multi_vector_inner_product(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g2.x) * other.g2 * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g2.y) * other.g2.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g2.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g2.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g2.x) * vec4(other.g3.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.y) * other.g3.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g3.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g3.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) - vec4(self.g3.x) * other.g2 + vec4(self.g3.y) * vec4(other.g2.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.z) * other.g2.wwxy * vec4(0.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g2.zzyx * vec4(0.0, 1.0, -1.0, -1.0) + vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.y) * other.g0.xxwz * vec4(0.0, 1.0, 1.0, -1.0) + vec4(self.g2.z) * other.g0.wwxy * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g2.w) * other.g0.zzyx * vec4(0.0, 1.0, -1.0, 1.0) + self.g0.xyyy * other.g2.xxwz * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g1.y) * other.g2.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * other.g2.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g2.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.z) * other.g1.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g2.w) * other.g1.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0 * vec4(other.g3.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

MultiVector multi_vector_multi_vector_left_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g2.x) * vec4(other.g2.x) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g2.y) * other.g2.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g2.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g2.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.x) * vec4(other.g3.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.y) * other.g3.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g3.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g3.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g2 + vec4(self.g2.y) * other.g0.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g2.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g2.x) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g2.y) * other.g1.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g2.z) * other.g1.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g2.w) * other.g1.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g3.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

MultiVector multi_vector_multi_vector_right_contraction(MultiVector self, MultiVector other) {
    return MultiVector(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g2.x) * other.g2 * vec4(-1.0, 1.0, 1.0, 1.0) + vec4(self.g2.y) * vec4(other.g2.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) - vec4(self.g3.x) * other.g2 + vec4(self.g3.y) * vec4(other.g2.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.z) * other.g2.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g2.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x, self.g2.y, self.g0.y, self.g0.y) * vec4(other.g2.x, other.g0.x, other.g2.w, other.g2.z) * vec4(0.0, 1.0, 1.0, -1.0), vec4(self.g1.z) * other.g2.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g2.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g3.y, self.g1.y, self.g1.y) * vec4(other.g2.x, other.g0.x, other.g2.w, other.g2.z) * vec4(0.0, 1.0, -1.0, 1.0));
}

Scalar multi_vector_multi_vector_scalar_product(MultiVector self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w - self.g2.x * other.g2.x + self.g2.y * other.g2.y + self.g2.z * other.g2.z + self.g2.w * other.g2.w);
}

Rotor multi_vector_rotor_into(MultiVector self) {
    return Rotor(self.g0);
}

MultiVector multi_vector_rotor_add(MultiVector self, Rotor other) {
    return MultiVector(self.g0 + other.g0, self.g1, self.g2, self.g3);
}

MultiVector multi_vector_rotor_sub(MultiVector self, Rotor other) {
    return MultiVector(self.g0 - other.g0, self.g1, self.g2, self.g3);
}

MultiVector multi_vector_rotor_geometric_product(MultiVector self, Rotor other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0), vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g2.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g2.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0), vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g3.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g3.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

MultiVector multi_vector_rotor_outer_product(MultiVector self, Rotor other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g1 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g2.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g2.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g3.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector multi_vector_rotor_inner_product(MultiVector self, Rotor other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.z) * other.g0.wwxy * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g2.w) * other.g0.zzyx * vec4(0.0, 1.0, -1.0, 1.0) + self.g2.xyyy * other.g0.xxwz * vec4(0.0, 1.0, 1.0, -1.0), vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g3 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_rotor_right_contraction(MultiVector self, Rotor other) {
    return MultiVector(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g2 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g3 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar multi_vector_rotor_scalar_product(MultiVector self, Rotor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Point multi_vector_point_into(MultiVector self) {
    return Point(vec4(self.g2.x, self.g1.y, self.g1.z, self.g1.w));
}

MultiVector multi_vector_point_add(MultiVector self, Point other) {
    return MultiVector(self.g0, self.g1 + other.g0 * vec4(0.0, 1.0, 1.0, 1.0), self.g2 + vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g3);
}

MultiVector multi_vector_point_sub(MultiVector self, Point other) {
    return MultiVector(self.g0, self.g1 - other.g0 * vec4(0.0, 1.0, 1.0, 1.0), self.g2 - vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g3);
}

MultiVector multi_vector_point_geometric_product(MultiVector self, Point other) {
    return MultiVector(self.g2 * vec4(other.g0.x) * vec4(-1.0, 1.0, 1.0, 1.0), vec4(self.g0.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g3.x, self.g0.x, self.g0.x, self.g0.x) * other.g0 * vec4(-1.0, 1.0, 1.0, 1.0), self.g0 * vec4(other.g0.x) * vec4(1.0, -1.0, -1.0, -1.0), vec4(self.g2.x) * other.g0.yyzw * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g2.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g2.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g2.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + self.g1 * vec4(other.g0.x));
}

Scalar multi_vector_point_scalar_product(MultiVector self, Point other) {
    return Scalar(0.0 - self.g2.x * other.g0.x);
}

Plane multi_vector_plane_into(MultiVector self) {
    return Plane(vec4(self.g1.x, self.g2.y, self.g2.z, self.g2.w));
}

MultiVector multi_vector_plane_add(MultiVector self, Plane other) {
    return MultiVector(self.g0, self.g1 + vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g2 + other.g0 * vec4(0.0, 1.0, 1.0, 1.0), self.g3);
}

MultiVector multi_vector_plane_sub(MultiVector self, Plane other) {
    return MultiVector(self.g0, self.g1 - vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g2 - other.g0 * vec4(0.0, 1.0, 1.0, 1.0), self.g3);
}

MultiVector multi_vector_plane_geometric_product(MultiVector self, Plane other) {
    return MultiVector(vec4(self.g2.y) * other.g0.yywz * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * other.g0.zwzy * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * other.g0.wzyw * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g2.x) * other.g0 * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g3.x) * other.g0.yyzw * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g3.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g3.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g0.x) * vec4(1.0, -1.0, -1.0, -1.0), vec4(self.g0.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * other.g0 * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g1.y) * other.g0.yywz * vec4(-1.0, 0.0, -1.0, 1.0) + vec4(self.g1.z) * other.g0.zwzy * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g1.w) * other.g0.wzyw * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g2.y) * vec4(other.g0.x) * vec4(0.0, -1.0, 0.0, 0.0) + vec4(self.g2.z) * vec4(other.g0.x) * vec4(0.0, 0.0, -1.0, 0.0) + vec4(self.g2.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, -1.0) + vec4(self.g2.x, self.g1.x, self.g1.x, self.g1.x) * other.g0 * vec4(-1.0, 1.0, 1.0, 1.0));
}

Scalar multi_vector_plane_scalar_product(MultiVector self, Plane other) {
    return Scalar(self.g2.y * other.g0.y + self.g2.z * other.g0.z + self.g2.w * other.g0.w);
}

Line multi_vector_line_into(MultiVector self) {
    return Line(vec3(self.g3.y, self.g3.z, self.g3.w), vec3(self.g0.y, self.g0.z, self.g0.w));
}

MultiVector multi_vector_line_add(MultiVector self, Line other) {
    return MultiVector(self.g0 + vec4(other.g0.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), self.g1, self.g2, self.g3 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_line_sub(MultiVector self, Line other) {
    return MultiVector(self.g0 - vec4(other.g0.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), self.g1, self.g2, self.g3 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_line_geometric_product(MultiVector self, Line other) {
    return MultiVector(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g1.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g2.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + vec4(self.g2.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(-1.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(-1.0, -1.0, 1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g2.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g2.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g2.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g2.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g3.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g3.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g3.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar multi_vector_line_scalar_product(MultiVector self, Line other) {
    return Scalar(0.0 - self.g0.y * other.g1.x - self.g0.z * other.g1.y - self.g0.w * other.g1.z);
}

Translator multi_vector_translator_into(MultiVector self) {
    return Translator(vec4(self.g0.x, self.g3.y, self.g3.z, self.g3.w));
}

MultiVector multi_vector_translator_add(MultiVector self, Translator other) {
    return MultiVector(self.g0 + vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g1, self.g2, self.g3 + other.g0 * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_translator_sub(MultiVector self, Translator other) {
    return MultiVector(self.g0 - vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g1, self.g2, self.g3 - other.g0 * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_translator_geometric_product(MultiVector self, Translator other) {
    return MultiVector(self.g0 * vec4(other.g0.x), vec4(self.g2.x) * other.g0.yyzw * vec4(0.0, 1.0, 1.0, 1.0) + vec4(self.g2.y) * other.g0.yywz * vec4(-1.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * other.g0.zwzy * vec4(-1.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * other.g0.wzyw * vec4(-1.0, -1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.x), self.g2 * vec4(other.g0.x), vec4(self.g0.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x, self.g0.x, self.g0.x, self.g0.x) * other.g0);
}

MultiVector multi_vector_translator_outer_product(MultiVector self, Translator other) {
    return MultiVector(self.g0 * vec4(other.g0.x), vec4(self.g2.y) * other.g0.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.x), self.g2 * vec4(other.g0.x), vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g0.yyzw);
}

MultiVector multi_vector_translator_inner_product(MultiVector self, Translator other) {
    return MultiVector(self.g0 * vec4(other.g0.x), vec4(self.g2.y) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g2.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g2.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g1 * vec4(other.g0.x), self.g2 * vec4(other.g0.x), vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g3.x, self.g0.x, self.g0.x, self.g0.x) * other.g0);
}

MultiVector multi_vector_translator_right_contraction(MultiVector self, Translator other) {
    return MultiVector(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x), self.g2 * vec4(other.g0.x), self.g3 * vec4(other.g0.x));
}

Scalar multi_vector_translator_scalar_product(MultiVector self, Translator other) {
    return Scalar(self.g0.x * other.g0.x);
}

Motor multi_vector_motor_into(MultiVector self) {
    return Motor(self.g0, self.g3);
}

MultiVector multi_vector_motor_add(MultiVector self, Motor other) {
    return MultiVector(self.g0 + other.g0, self.g1, self.g2, self.g3 + other.g1);
}

MultiVector multi_vector_motor_sub(MultiVector self, Motor other) {
    return MultiVector(self.g0 - other.g0, self.g1, self.g2, self.g3 - other.g1);
}

MultiVector multi_vector_motor_geometric_product(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g2.x) * other.g1 + vec4(self.g2.y) * other.g1.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g2.z) * other.g1.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g2.w) * other.g1.wzyx * vec4(-1.0, -1.0, 1.0, 1.0), vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g2.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g2.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g3.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g3.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

MultiVector multi_vector_motor_regressive_product(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.y) * other.g1.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g1.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g3.x) * other.g0 + vec4(self.g3.y) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.y) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g1.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g1.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g2.x) * other.g1 + vec4(self.g2.z) * vec4(other.g1.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * vec4(other.g1.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g2.y, self.g1.y, self.g1.y) * vec4(other.g0.x, other.g1.x, other.g0.w, other.g0.z) * vec4(0.0, 1.0, 1.0, -1.0), vec4(self.g3.x) * other.g1 + self.g3 * vec4(other.g1.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_motor_outer_product(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.y) * other.g1.wwwz * vec4(0.0, 0.0, -1.0, 1.0) + vec4(self.g2.z) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g2.w) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + self.g1 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g2.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g2.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g3.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector multi_vector_motor_inner_product(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g2.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g2.y) * other.g1.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g2.z) * other.g1.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g2.w) * other.g1.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g2.z) * other.g0.wwxy * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g2.w) * other.g0.zzyx * vec4(0.0, 1.0, -1.0, 1.0) + self.g2.xyyy * other.g0.xxwz * vec4(0.0, 1.0, 1.0, -1.0), vec4(self.g0.x) * other.g1 + vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g3.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g3.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g3.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

MultiVector multi_vector_motor_right_contraction(MultiVector self, Motor other) {
    return MultiVector(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g2.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g2 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g3.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g3 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar multi_vector_motor_scalar_product(MultiVector self, Motor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Branch multi_vector_branch_into(MultiVector self) {
    return Branch(vec3(self.g3.y, self.g3.z, self.g3.w));
}

MultiVector multi_vector_branch_add(MultiVector self, Branch other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector multi_vector_branch_sub(MultiVector self, Branch other) {
    return MultiVector(self.g0, self.g1, self.g2, self.g3 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar multi_vector_squared_magnitude(MultiVector self) {
    return multi_vector_multi_vector_scalar_product(self, multi_vector_reversal(self));
}

Scalar multi_vector_magnitude(MultiVector self) {
    return Scalar(sqrt(multi_vector_squared_magnitude(self).g0));
}

MultiVector multi_vector_signum(MultiVector self) {
    return multi_vector_scalar_geometric_product(self, Scalar(1.0 / multi_vector_magnitude(self).g0));
}

MultiVector multi_vector_inverse(MultiVector self) {
    return multi_vector_scalar_geometric_product(multi_vector_reversal(self), Scalar(1.0 / multi_vector_squared_magnitude(self).g0));
}

Rotor rotor_zero() {
    return Rotor(vec4(0.0));
}

Rotor rotor_one() {
    return Rotor(vec4(1.0, 0.0, 0.0, 0.0));
}

Rotor rotor_neg(Rotor self) {
    return Rotor(self.g0 * vec4(-1.0));
}

Rotor rotor_automorphism(Rotor self) {
    return Rotor(self.g0);
}

Rotor rotor_reversal(Rotor self) {
    return Rotor(self.g0 * vec4(1.0, -1.0, -1.0, -1.0));
}

Rotor rotor_conjugation(Rotor self) {
    return Rotor(self.g0 * vec4(1.0, -1.0, -1.0, -1.0));
}

Scalar rotor_scalar_into(Rotor self) {
    return Scalar(self.g0.x);
}

Rotor rotor_scalar_add(Rotor self, Scalar other) {
    return Rotor(self.g0 + vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Rotor rotor_scalar_sub(Rotor self, Scalar other) {
    return Rotor(self.g0 - vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Rotor rotor_scalar_geometric_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Rotor rotor_scalar_outer_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Rotor rotor_scalar_inner_product(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Scalar rotor_scalar_left_contraction(Rotor self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

Rotor rotor_scalar_right_contraction(Rotor self, Scalar other) {
    return Rotor(self.g0 * vec4(other.g0));
}

Scalar rotor_scalar_scalar_product(Rotor self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector rotor_multi_vector_add(Rotor self, MultiVector other) {
    return MultiVector(self.g0 + other.g0, other.g1, other.g2, other.g3);
}

MultiVector rotor_multi_vector_sub(Rotor self, MultiVector other) {
    return MultiVector(self.g0 - other.g0, vec4(0.0) - other.g1, vec4(0.0) - other.g2, vec4(0.0) - other.g3);
}

MultiVector rotor_multi_vector_geometric_product(Rotor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.y) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.y) * other.g3.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g3.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g3.wzyx * vec4(1.0, 1.0, -1.0, -1.0));
}

MultiVector rotor_multi_vector_outer_product(Rotor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g2.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.z) * vec4(other.g3.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g3.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g3.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector rotor_multi_vector_inner_product(Rotor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.z) * other.g2.wwxy * vec4(0.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g2.zzyx * vec4(0.0, 1.0, -1.0, -1.0) + self.g0.xyyy * other.g2.xxwz * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g0.x) * other.g3 + self.g0 * vec4(other.g3.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

MultiVector rotor_multi_vector_left_contraction(Rotor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g2 + self.g0 * vec4(other.g2.x) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.x) * other.g3 + self.g0 * vec4(other.g3.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

Scalar rotor_multi_vector_scalar_product(Rotor self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Rotor rotor_rotor_add(Rotor self, Rotor other) {
    return Rotor(self.g0 + other.g0);
}

Rotor rotor_rotor_sub(Rotor self, Rotor other) {
    return Rotor(self.g0 - other.g0);
}

Rotor rotor_rotor_geometric_product(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0));
}

Rotor rotor_rotor_outer_product(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Rotor rotor_rotor_inner_product(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

Rotor rotor_rotor_left_contraction(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

Rotor rotor_rotor_right_contraction(Rotor self, Rotor other) {
    return Rotor(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Scalar rotor_rotor_scalar_product(Rotor self, Rotor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Point rotor_point_outer_product(Rotor self, Point other) {
    return Point(vec4(self.g0.x) * other.g0);
}

Plane rotor_plane_inner_product(Rotor self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0.xxyy * other.g0.xxwz * vec4(0.0, 0.0, 1.0, -1.0));
}

Plane rotor_plane_left_contraction(Rotor self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

Motor rotor_line_geometric_product(Rotor self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar rotor_line_regressive_product(Rotor self, Line other) {
    return Scalar(self.g0.y * other.g0.x + self.g0.z * other.g0.y + self.g0.w * other.g0.z);
}

Scalar rotor_line_right_contraction(Rotor self, Line other) {
    return Scalar(0.0 - self.g0.y * other.g1.x - self.g0.z * other.g1.y - self.g0.w * other.g1.z);
}

Scalar rotor_line_scalar_product(Rotor self, Line other) {
    return Scalar(0.0 - self.g0.y * other.g1.x - self.g0.z * other.g1.y - self.g0.w * other.g1.z);
}

Motor rotor_translator_geometric_product(Rotor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.x), vec4(self.g0.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * other.g0 * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar rotor_translator_regressive_product(Rotor self, Translator other) {
    return Scalar(self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Motor rotor_translator_outer_product(Rotor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.x), vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yyzw);
}

Translator rotor_translator_left_contraction(Rotor self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0);
}

Rotor rotor_translator_right_contraction(Rotor self, Translator other) {
    return Rotor(self.g0 * vec4(other.g0.x));
}

Scalar rotor_translator_scalar_product(Rotor self, Translator other) {
    return Scalar(self.g0.x * other.g0.x);
}

Motor rotor_motor_add(Rotor self, Motor other) {
    return Motor(self.g0 + other.g0, other.g1);
}

Motor rotor_motor_sub(Rotor self, Motor other) {
    return Motor(self.g0 - other.g0, vec4(0.0) - other.g1);
}

Motor rotor_motor_geometric_product(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0));
}

Rotor rotor_motor_regressive_product(Rotor self, Motor other) {
    return Rotor(vec4(self.g0.y) * other.g1.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g1.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor rotor_motor_outer_product(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor rotor_motor_inner_product(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

Motor rotor_motor_left_contraction(Rotor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

Rotor rotor_motor_right_contraction(Rotor self, Motor other) {
    return Rotor(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Scalar rotor_motor_scalar_product(Rotor self, Motor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Scalar rotor_branch_regressive_product(Rotor self, Branch other) {
    return Scalar(self.g0.y * other.g0.x + self.g0.z * other.g0.y + self.g0.w * other.g0.z);
}

Branch rotor_branch_inner_product(Rotor self, Branch other) {
    return Branch(vec3(self.g0.x) * other.g0);
}

Branch rotor_branch_left_contraction(Rotor self, Branch other) {
    return Branch(vec3(self.g0.x) * other.g0);
}

Scalar rotor_squared_magnitude(Rotor self) {
    return rotor_rotor_scalar_product(self, rotor_reversal(self));
}

Scalar rotor_magnitude(Rotor self) {
    return Scalar(sqrt(rotor_squared_magnitude(self).g0));
}

Rotor rotor_signum(Rotor self) {
    return rotor_scalar_geometric_product(self, Scalar(1.0 / rotor_magnitude(self).g0));
}

Rotor rotor_inverse(Rotor self) {
    return rotor_scalar_geometric_product(rotor_reversal(self), Scalar(1.0 / rotor_squared_magnitude(self).g0));
}

Point point_zero() {
    return Point(vec4(0.0));
}

Point point_one() {
    return Point(vec4(0.0));
}

Point point_neg(Point self) {
    return Point(self.g0 * vec4(-1.0));
}

Point point_automorphism(Point self) {
    return Point(self.g0 * vec4(-1.0));
}

Point point_reversal(Point self) {
    return Point(self.g0 * vec4(-1.0));
}

Point point_conjugation(Point self) {
    return Point(self.g0);
}

Plane point_dual(Point self) {
    return Plane(self.g0 * vec4(-1.0));
}

Point point_scalar_geometric_product(Point self, Scalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Point point_scalar_outer_product(Point self, Scalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Point point_scalar_inner_product(Point self, Scalar other) {
    return Point(self.g0 * vec4(other.g0));
}

Point point_scalar_right_contraction(Point self, Scalar other) {
    return Point(self.g0 * vec4(other.g0));
}

MultiVector point_multi_vector_add(Point self, MultiVector other) {
    return MultiVector(other.g0, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) + other.g1, vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + other.g2, other.g3);
}

MultiVector point_multi_vector_sub(Point self, MultiVector other) {
    return MultiVector(vec4(0.0) - other.g0, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) - other.g1, vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) - other.g2, vec4(0.0) - other.g3);
}

MultiVector point_multi_vector_geometric_product(Point self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g2 * vec4(-1.0, 1.0, 1.0, 1.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0), vec4(0.0) - vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g2.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g2.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g2.wzyx * vec4(-1.0, -1.0, 1.0, 1.0));
}

Scalar point_multi_vector_scalar_product(Point self, MultiVector other) {
    return Scalar(0.0 - self.g0.x * other.g2.x);
}

Point point_rotor_outer_product(Point self, Rotor other) {
    return Point(self.g0 * vec4(other.g0.x));
}

Point point_point_add(Point self, Point other) {
    return Point(self.g0 + other.g0);
}

Point point_point_sub(Point self, Point other) {
    return Point(self.g0 - other.g0);
}

Translator point_point_geometric_product(Point self, Point other) {
    return Translator(vec4(0.0) - vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Line point_point_regressive_product(Point self, Point other) {
    return Line(vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * vec3(other.g0.y, other.g0.z, other.g0.w) + vec3(self.g0.y, self.g0.z, self.g0.w) * vec3(other.g0.x) * vec3(-1.0));
}

Scalar point_point_inner_product(Point self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x);
}

Scalar point_point_left_contraction(Point self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x);
}

Scalar point_point_right_contraction(Point self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x);
}

Scalar point_point_scalar_product(Point self, Point other) {
    return Scalar(0.0 - self.g0.x * other.g0.x);
}

Scalar point_plane_regressive_product(Point self, Plane other) {
    return Scalar(0.0 - self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Line point_plane_inner_product(Point self, Plane other) {
    return Line(vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * vec3(other.g0.y, other.g0.z, other.g0.w));
}

Line point_plane_right_contraction(Point self, Plane other) {
    return Line(vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, -1.0, 1.0), vec3(self.g0.x) * vec3(other.g0.y, other.g0.z, other.g0.w));
}

Plane point_line_regressive_product(Point self, Line other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Plane point_line_inner_product(Point self, Line other) {
    return Plane(vec4(self.g0.z) * vec4(other.g1.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, -1.0, -1.0));
}

Plane point_line_right_contraction(Point self, Line other) {
    return Plane(vec4(self.g0.z) * vec4(other.g1.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(1.0, -1.0, -1.0, -1.0));
}

Point point_translator_geometric_product(Point self, Translator other) {
    return Point(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Plane point_translator_regressive_product(Point self, Translator other) {
    return Plane(vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yyzw * vec4(-1.0, 1.0, 1.0, 1.0));
}

Point point_translator_outer_product(Point self, Translator other) {
    return Point(self.g0 * vec4(other.g0.x));
}

Point point_translator_inner_product(Point self, Translator other) {
    return Point(self.g0 * vec4(other.g0.x));
}

Point point_translator_right_contraction(Point self, Translator other) {
    return Point(self.g0 * vec4(other.g0.x));
}

Point point_motor_outer_product(Point self, Motor other) {
    return Point(self.g0 * vec4(other.g0.x));
}

Plane point_branch_regressive_product(Point self, Branch other) {
    return Plane(vec4(self.g0.z) * vec4(other.g0.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(-1.0, 1.0, 1.0, 1.0));
}

Scalar point_squared_magnitude(Point self) {
    return point_point_scalar_product(self, point_reversal(self));
}

Scalar point_magnitude(Point self) {
    return Scalar(sqrt(point_squared_magnitude(self).g0));
}

Point point_signum(Point self) {
    return point_scalar_geometric_product(self, Scalar(1.0 / point_magnitude(self).g0));
}

Point point_inverse(Point self) {
    return point_scalar_geometric_product(point_reversal(self), Scalar(1.0 / point_squared_magnitude(self).g0));
}

Plane plane_zero() {
    return Plane(vec4(0.0));
}

Plane plane_one() {
    return Plane(vec4(0.0));
}

Plane plane_neg(Plane self) {
    return Plane(self.g0 * vec4(-1.0));
}

Plane plane_automorphism(Plane self) {
    return Plane(self.g0 * vec4(-1.0));
}

Plane plane_reversal(Plane self) {
    return Plane(self.g0);
}

Plane plane_conjugation(Plane self) {
    return Plane(self.g0 * vec4(-1.0));
}

Point plane_dual(Plane self) {
    return Point(self.g0);
}

Plane plane_scalar_geometric_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_scalar_outer_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_scalar_inner_product(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

Plane plane_scalar_right_contraction(Plane self, Scalar other) {
    return Plane(self.g0 * vec4(other.g0));
}

MultiVector plane_multi_vector_add(Plane self, MultiVector other) {
    return MultiVector(other.g0, vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + other.g1, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) + other.g2, other.g3);
}

MultiVector plane_multi_vector_sub(Plane self, MultiVector other) {
    return MultiVector(vec4(0.0) - other.g0, vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) - other.g1, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) - other.g2, vec4(0.0) - other.g3);
}

MultiVector plane_multi_vector_geometric_product(Plane self, MultiVector other) {
    return MultiVector(vec4(self.g0.y) * other.g2.yxwz * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g2.zwxy * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g2.wzyx * vec4(1.0, -1.0, 1.0, 1.0), vec4(self.g0.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g0.y) * other.g3.yxwz * vec4(-1.0, 1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g3.zwxy * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.w) * other.g3.wzyx * vec4(-1.0, -1.0, 1.0, 1.0), vec4(self.g0.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0));
}

Scalar plane_multi_vector_scalar_product(Plane self, MultiVector other) {
    return Scalar(self.g0.y * other.g2.y + self.g0.z * other.g2.z + self.g0.w * other.g2.w);
}

Plane plane_rotor_inner_product(Plane self, Rotor other) {
    return Plane(vec4(self.g0.z) * other.g0.wwxy * vec4(0.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.zzyx * vec4(0.0, 1.0, -1.0, 1.0) + self.g0.xyyy * other.g0.xxwz * vec4(1.0, 1.0, 1.0, -1.0));
}

Plane plane_rotor_right_contraction(Plane self, Rotor other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

Scalar plane_point_regressive_product(Plane self, Point other) {
    return Scalar(self.g0.x * other.g0.x + self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Line plane_point_inner_product(Plane self, Point other) {
    return Line(vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, 1.0, -1.0), vec3(self.g0.y, self.g0.z, self.g0.w) * vec3(other.g0.x));
}

Line plane_point_left_contraction(Plane self, Point other) {
    return Line(vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(-1.0, 0.0, 1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(1.0, -1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, 1.0, -1.0), vec3(self.g0.y, self.g0.z, self.g0.w) * vec3(other.g0.x));
}

Plane plane_plane_add(Plane self, Plane other) {
    return Plane(self.g0 + other.g0);
}

Plane plane_plane_sub(Plane self, Plane other) {
    return Plane(self.g0 - other.g0);
}

Line plane_plane_outer_product(Plane self, Plane other) {
    return Line(vec3(self.g0.x) * vec3(other.g0.y, other.g0.z, other.g0.w) + vec3(self.g0.y, self.g0.z, self.g0.w) * vec3(other.g0.x) * vec3(-1.0), vec3(self.g0.z) * vec3(other.g0.w, other.g0.w, other.g0.y) * vec3(1.0, 0.0, -1.0) + vec3(self.g0.w) * vec3(other.g0.z, other.g0.y, other.g0.z) * vec3(-1.0, 1.0, 0.0) + vec3(self.g0.x, self.g0.y, self.g0.y) * vec3(other.g0.x, other.g0.w, other.g0.z) * vec3(0.0, -1.0, 1.0));
}

Scalar plane_plane_inner_product(Plane self, Plane other) {
    return Scalar(self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Scalar plane_plane_left_contraction(Plane self, Plane other) {
    return Scalar(self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Scalar plane_plane_right_contraction(Plane self, Plane other) {
    return Scalar(self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Scalar plane_plane_scalar_product(Plane self, Plane other) {
    return Scalar(self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Point plane_line_outer_product(Plane self, Line other) {
    return Point(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, -1.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g0.z, other.g1.y, other.g0.x) * vec4(1.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g0.y, other.g0.x, other.g1.z) * vec4(1.0, -1.0, 1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0));
}

Plane plane_line_inner_product(Plane self, Line other) {
    return Plane(vec4(self.g0.z) * vec4(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + self.g0.yxyy * vec4(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0));
}

Plane plane_line_left_contraction(Plane self, Line other) {
    return Plane(vec4(self.g0.z) * vec4(other.g0.y, other.g1.z, other.g0.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g1.y, other.g1.x, other.g0.z) * vec4(-1.0, 1.0, -1.0, 0.0) + self.g0.yxyy * vec4(other.g0.x, other.g0.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0));
}

Plane plane_translator_inner_product(Plane self, Translator other) {
    return Plane(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Plane plane_translator_right_contraction(Plane self, Translator other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

Plane plane_motor_regressive_product(Plane self, Motor other) {
    return Plane(self.g0 * vec4(other.g1.x));
}

Plane plane_motor_right_contraction(Plane self, Motor other) {
    return Plane(self.g0 * vec4(other.g0.x));
}

Scalar plane_squared_magnitude(Plane self) {
    return plane_plane_scalar_product(self, plane_reversal(self));
}

Scalar plane_magnitude(Plane self) {
    return Scalar(sqrt(plane_squared_magnitude(self).g0));
}

Plane plane_signum(Plane self) {
    return plane_scalar_geometric_product(self, Scalar(1.0 / plane_magnitude(self).g0));
}

Plane plane_inverse(Plane self) {
    return plane_scalar_geometric_product(plane_reversal(self), Scalar(1.0 / plane_squared_magnitude(self).g0));
}

Line line_zero() {
    return Line(vec3(0.0), vec3(0.0));
}

Line line_one() {
    return Line(vec3(0.0), vec3(0.0));
}

Line line_neg(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Line line_automorphism(Line self) {
    return Line(self.g0, self.g1);
}

Line line_reversal(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Line line_conjugation(Line self) {
    return Line(self.g0 * vec3(-1.0), self.g1 * vec3(-1.0));
}

Line line_dual(Line self) {
    return Line(self.g1, self.g0);
}

Line line_scalar_geometric_product(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_scalar_outer_product(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_scalar_inner_product(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

Line line_scalar_right_contraction(Line self, Scalar other) {
    return Line(self.g0 * vec3(other.g0), self.g1 * vec3(other.g0));
}

MultiVector line_multi_vector_add(Line self, MultiVector other) {
    return MultiVector(vec4(self.g0.x, self.g1.x, self.g1.y, self.g1.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g0, other.g1, other.g2, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g3);
}

MultiVector line_multi_vector_sub(Line self, MultiVector other) {
    return MultiVector(vec4(self.g0.x, self.g1.x, self.g1.y, self.g1.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g0, vec4(0.0) - other.g1, vec4(0.0) - other.g2, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g3);
}

MultiVector line_multi_vector_geometric_product(Line self, MultiVector other) {
    return MultiVector(vec4(self.g1.x) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g1.z) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.z) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g1.x) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.x) * other.g3.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g3.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g3.wzyx * vec4(1.0, 1.0, -1.0, -1.0));
}

Scalar line_multi_vector_scalar_product(Line self, MultiVector other) {
    return Scalar(0.0 - self.g1.x * other.g0.y - self.g1.y * other.g0.z - self.g1.z * other.g0.w);
}

Motor line_rotor_geometric_product(Line self, Rotor other) {
    return Motor(vec4(self.g1.x) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g1.z) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

Scalar line_rotor_regressive_product(Line self, Rotor other) {
    return Scalar(self.g0.x * other.g0.y + self.g0.y * other.g0.z + self.g0.z * other.g0.w);
}

Scalar line_rotor_left_contraction(Line self, Rotor other) {
    return Scalar(0.0 - self.g1.x * other.g0.y - self.g1.y * other.g0.z - self.g1.z * other.g0.w);
}

Scalar line_rotor_scalar_product(Line self, Rotor other) {
    return Scalar(0.0 - self.g1.x * other.g0.y - self.g1.y * other.g0.z - self.g1.z * other.g0.w);
}

Plane line_point_regressive_product(Line self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g1.z) * other.g0.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g1.x, self.g1.x) * other.g0.yxwz * vec4(-1.0, 1.0, -1.0, 1.0));
}

Plane line_point_inner_product(Line self, Point other) {
    return Plane(vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * other.g0.yxxx * vec4(1.0, -1.0, 0.0, 0.0));
}

Plane line_point_left_contraction(Line self, Point other) {
    return Plane(vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * other.g0.yxxx * vec4(1.0, -1.0, 0.0, 0.0));
}

Point line_plane_outer_product(Line self, Plane other) {
    return Point(vec4(self.g0.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * other.g0.zzxz * vec4(1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, -1.0) + vec4(self.g1.x, self.g1.x, self.g0.x, self.g0.x) * other.g0.yxwz * vec4(1.0, -1.0, 1.0, -1.0));
}

Plane line_plane_inner_product(Line self, Plane other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g1.x, self.g1.x) * other.g0.yxwz * vec4(1.0, 0.0, 1.0, -1.0));
}

Plane line_plane_right_contraction(Line self, Plane other) {
    return Plane(vec4(self.g0.y) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * other.g0.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + vec4(self.g0.x, self.g0.x, self.g1.x, self.g1.x) * other.g0.yxwz * vec4(1.0, 0.0, 1.0, -1.0));
}

Line line_line_add(Line self, Line other) {
    return Line(self.g0 + other.g0, self.g1 + other.g1);
}

Line line_line_sub(Line self, Line other) {
    return Line(self.g0 - other.g0, self.g1 - other.g1);
}

Motor line_line_geometric_product(Line self, Line other) {
    return Motor(vec4(self.g1.y) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0), vec4(self.g0.y) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.z) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g1.z) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(1.0, 0.0, 1.0, -1.0));
}

Scalar line_line_regressive_product(Line self, Line other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z + self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

Scalar line_line_inner_product(Line self, Line other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar line_line_left_contraction(Line self, Line other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar line_line_right_contraction(Line self, Line other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar line_line_scalar_product(Line self, Line other) {
    return Scalar(0.0 - self.g1.x * other.g1.x - self.g1.y * other.g1.y - self.g1.z * other.g1.z);
}

Scalar line_translator_regressive_product(Line self, Translator other) {
    return Scalar(self.g1.x * other.g0.y + self.g1.y * other.g0.z + self.g1.z * other.g0.w);
}

Line line_translator_inner_product(Line self, Translator other) {
    return Line(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x));
}

Line line_translator_right_contraction(Line self, Translator other) {
    return Line(self.g0 * vec3(other.g0.x), self.g1 * vec3(other.g0.x));
}

Motor line_motor_add(Line self, Motor other) {
    return Motor(vec4(self.g0.x, self.g1.x, self.g1.y, self.g1.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g0, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g1);
}

Motor line_motor_sub(Line self, Motor other) {
    return Motor(vec4(self.g0.x, self.g1.x, self.g1.y, self.g1.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g0, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g1);
}

Motor line_motor_geometric_product(Line self, Motor other) {
    return Motor(vec4(self.g1.x) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g1.z) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.y) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.z) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0) + vec4(self.g1.x) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.y) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.z) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0));
}

Translator line_motor_left_contraction(Line self, Motor other) {
    return Translator(vec4(self.g1.y) * vec4(other.g0.z, other.g0.z, other.g1.x, other.g0.z) * vec4(-1.0, 0.0, -1.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.x) * vec4(-1.0, 0.0, 0.0, -1.0) + vec4(self.g1.x) * vec4(other.g0.y, other.g1.x, other.g0.x, other.g0.x) * vec4(-1.0, -1.0, 0.0, 0.0));
}

Scalar line_motor_scalar_product(Line self, Motor other) {
    return Scalar(0.0 - self.g1.x * other.g0.y - self.g1.y * other.g0.z - self.g1.z * other.g0.w);
}

Branch line_branch_into(Line self) {
    return Branch(self.g0);
}

Line line_branch_add(Line self, Branch other) {
    return Line(self.g0 + other.g0, self.g1);
}

Line line_branch_sub(Line self, Branch other) {
    return Line(self.g0 - other.g0, self.g1);
}

Scalar line_branch_regressive_product(Line self, Branch other) {
    return Scalar(self.g1.x * other.g0.x + self.g1.y * other.g0.y + self.g1.z * other.g0.z);
}

Scalar line_squared_magnitude(Line self) {
    return line_line_scalar_product(self, line_reversal(self));
}

Scalar line_magnitude(Line self) {
    return Scalar(sqrt(line_squared_magnitude(self).g0));
}

Line line_signum(Line self) {
    return line_scalar_geometric_product(self, Scalar(1.0 / line_magnitude(self).g0));
}

Line line_inverse(Line self) {
    return line_scalar_geometric_product(line_reversal(self), Scalar(1.0 / line_squared_magnitude(self).g0));
}

Translator translator_zero() {
    return Translator(vec4(0.0));
}

Translator translator_one() {
    return Translator(vec4(1.0, 0.0, 0.0, 0.0));
}

Translator translator_neg(Translator self) {
    return Translator(self.g0 * vec4(-1.0));
}

Translator translator_automorphism(Translator self) {
    return Translator(self.g0);
}

Translator translator_reversal(Translator self) {
    return Translator(self.g0 * vec4(1.0, -1.0, -1.0, -1.0));
}

Translator translator_conjugation(Translator self) {
    return Translator(self.g0 * vec4(1.0, -1.0, -1.0, -1.0));
}

Scalar translator_scalar_into(Translator self) {
    return Scalar(self.g0.x);
}

Translator translator_scalar_add(Translator self, Scalar other) {
    return Translator(self.g0 + vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Translator translator_scalar_sub(Translator self, Scalar other) {
    return Translator(self.g0 - vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Translator translator_scalar_geometric_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Translator translator_scalar_outer_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Translator translator_scalar_inner_product(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Scalar translator_scalar_left_contraction(Translator self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

Translator translator_scalar_right_contraction(Translator self, Scalar other) {
    return Translator(self.g0 * vec4(other.g0));
}

Scalar translator_scalar_scalar_product(Translator self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector translator_multi_vector_add(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0, other.g1, other.g2, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) + other.g3);
}

MultiVector translator_multi_vector_sub(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0, vec4(0.0) - other.g1, vec4(0.0) - other.g2, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) - other.g3);
}

MultiVector translator_multi_vector_geometric_product(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2, vec4(self.g0.x) * other.g3 + vec4(self.g0.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

MultiVector translator_multi_vector_outer_product(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * other.g2.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g2.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0.xxyy * other.g2.xxwz * vec4(0.0, 0.0, 1.0, -1.0), vec4(self.g0.x) * other.g2, vec4(self.g0.x) * other.g3 + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0));
}

MultiVector translator_multi_vector_inner_product(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g2.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g2, vec4(self.g0.x) * other.g3 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector translator_multi_vector_left_contraction(Translator self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1, vec4(self.g0.x) * other.g2, vec4(self.g0.x) * other.g3);
}

Scalar translator_multi_vector_scalar_product(Translator self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x);
}

Motor translator_rotor_geometric_product(Translator self, Rotor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec4(self.g0.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

Scalar translator_rotor_regressive_product(Translator self, Rotor other) {
    return Scalar(self.g0.y * other.g0.y + self.g0.z * other.g0.z + self.g0.w * other.g0.w);
}

Motor translator_rotor_outer_product(Translator self, Rotor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0));
}

Rotor translator_rotor_left_contraction(Translator self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0);
}

Translator translator_rotor_right_contraction(Translator self, Rotor other) {
    return Translator(self.g0 * vec4(other.g0.x));
}

Scalar translator_rotor_scalar_product(Translator self, Rotor other) {
    return Scalar(self.g0.x * other.g0.x);
}

Point translator_point_geometric_product(Translator self, Point other) {
    return Point(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

Plane translator_point_regressive_product(Translator self, Point other) {
    return Plane(vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

Point translator_point_outer_product(Translator self, Point other) {
    return Point(vec4(self.g0.x) * other.g0);
}

Point translator_point_inner_product(Translator self, Point other) {
    return Point(vec4(self.g0.x) * other.g0);
}

Point translator_point_left_contraction(Translator self, Point other) {
    return Point(vec4(self.g0.x) * other.g0);
}

Plane translator_plane_inner_product(Translator self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

Plane translator_plane_left_contraction(Translator self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

Scalar translator_line_regressive_product(Translator self, Line other) {
    return Scalar(self.g0.y * other.g1.x + self.g0.z * other.g1.y + self.g0.w * other.g1.z);
}

Line translator_line_inner_product(Translator self, Line other) {
    return Line(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Line translator_line_left_contraction(Translator self, Line other) {
    return Line(vec3(self.g0.x) * other.g0, vec3(self.g0.x) * other.g1);
}

Translator translator_translator_add(Translator self, Translator other) {
    return Translator(self.g0 + other.g0);
}

Translator translator_translator_sub(Translator self, Translator other) {
    return Translator(self.g0 - other.g0);
}

Translator translator_translator_geometric_product(Translator self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Translator translator_translator_outer_product(Translator self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Translator translator_translator_inner_product(Translator self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Translator translator_translator_left_contraction(Translator self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0);
}

Translator translator_translator_right_contraction(Translator self, Translator other) {
    return Translator(self.g0 * vec4(other.g0.x));
}

Scalar translator_translator_scalar_product(Translator self, Translator other) {
    return Scalar(self.g0.x * other.g0.x);
}

Motor translator_motor_add(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + other.g0, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) + other.g1);
}

Motor translator_motor_sub(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) - other.g0, self.g0 * vec4(0.0, 1.0, 1.0, 1.0) - other.g1);
}

Motor translator_motor_geometric_product(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

Translator translator_motor_regressive_product(Translator self, Motor other) {
    return Translator(vec4(self.g0.y) * vec4(other.g0.y, other.g1.x, other.g0.y, other.g0.y) * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.z, other.g0.z, other.g1.x, other.g0.z) * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.x) * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor translator_motor_outer_product(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(1.0, 1.0, 0.0, 0.0));
}

Motor translator_motor_inner_product(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor translator_motor_left_contraction(Translator self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0, vec4(self.g0.x) * other.g1);
}

Translator translator_motor_right_contraction(Translator self, Motor other) {
    return Translator(self.g0 * vec4(other.g0.x));
}

Scalar translator_motor_scalar_product(Translator self, Motor other) {
    return Scalar(self.g0.x * other.g0.x);
}

Branch translator_branch_into(Translator self) {
    return Branch(vec3(self.g0.y, self.g0.z, self.g0.w));
}

Translator translator_branch_add(Translator self, Branch other) {
    return Translator(self.g0 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Translator translator_branch_sub(Translator self, Branch other) {
    return Translator(self.g0 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Branch translator_branch_geometric_product(Translator self, Branch other) {
    return Branch(vec3(self.g0.x) * other.g0);
}

Branch translator_branch_outer_product(Translator self, Branch other) {
    return Branch(vec3(self.g0.x) * other.g0);
}

Branch translator_branch_inner_product(Translator self, Branch other) {
    return Branch(vec3(self.g0.x) * other.g0);
}

Branch translator_branch_left_contraction(Translator self, Branch other) {
    return Branch(vec3(self.g0.x) * other.g0);
}

Scalar translator_squared_magnitude(Translator self) {
    return translator_translator_scalar_product(self, translator_reversal(self));
}

Scalar translator_magnitude(Translator self) {
    return Scalar(sqrt(translator_squared_magnitude(self).g0));
}

Translator translator_signum(Translator self) {
    return translator_scalar_geometric_product(self, Scalar(1.0 / translator_magnitude(self).g0));
}

Translator translator_inverse(Translator self) {
    return translator_scalar_geometric_product(translator_reversal(self), Scalar(1.0 / translator_squared_magnitude(self).g0));
}

Motor motor_zero() {
    return Motor(vec4(0.0), vec4(0.0));
}

Motor motor_one() {
    return Motor(vec4(1.0, 0.0, 0.0, 0.0), vec4(0.0));
}

Motor motor_neg(Motor self) {
    return Motor(self.g0 * vec4(-1.0), self.g1 * vec4(-1.0));
}

Motor motor_automorphism(Motor self) {
    return Motor(self.g0, self.g1);
}

Motor motor_reversal(Motor self) {
    return Motor(self.g0 * vec4(1.0, -1.0, -1.0, -1.0), self.g1 * vec4(1.0, -1.0, -1.0, -1.0));
}

Motor motor_conjugation(Motor self) {
    return Motor(self.g0 * vec4(1.0, -1.0, -1.0, -1.0), self.g1 * vec4(1.0, -1.0, -1.0, -1.0));
}

Motor motor_dual(Motor self) {
    return Motor(self.g1, self.g0);
}

Scalar motor_scalar_into(Motor self) {
    return Scalar(self.g0.x);
}

Motor motor_scalar_add(Motor self, Scalar other) {
    return Motor(self.g0 + vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0), self.g1);
}

Motor motor_scalar_sub(Motor self, Scalar other) {
    return Motor(self.g0 - vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0), self.g1);
}

Motor motor_scalar_geometric_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Scalar motor_scalar_regressive_product(Motor self, Scalar other) {
    return Scalar(self.g1.x * other.g0);
}

Motor motor_scalar_outer_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Motor motor_scalar_inner_product(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Scalar motor_scalar_left_contraction(Motor self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

Motor motor_scalar_right_contraction(Motor self, Scalar other) {
    return Motor(self.g0 * vec4(other.g0), self.g1 * vec4(other.g0));
}

Scalar motor_scalar_scalar_product(Motor self, Scalar other) {
    return Scalar(self.g0.x * other.g0);
}

MultiVector motor_multi_vector_add(Motor self, MultiVector other) {
    return MultiVector(self.g0 + other.g0, other.g1, other.g2, self.g1 + other.g3);
}

MultiVector motor_multi_vector_sub(Motor self, MultiVector other) {
    return MultiVector(self.g0 - other.g0, vec4(0.0) - other.g1, vec4(0.0) - other.g2, self.g1 - other.g3);
}

MultiVector motor_multi_vector_geometric_product(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0) - vec4(self.g1.x) * other.g2 + vec4(self.g1.y) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g1.w) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.y) * other.g2.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g2.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g2.wzyx * vec4(1.0, 1.0, -1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.y) * other.g3.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g3.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g3.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

MultiVector motor_multi_vector_regressive_product(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.y) * other.g3.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g3.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g3.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g0 + vec4(self.g1.y) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g3.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.x) * other.g1 + vec4(self.g1.z) * vec4(other.g1.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g1.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g1.yxxx * other.g1.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.z) * other.g1.wwwy * vec4(0.0, 1.0, 0.0, -1.0) + vec4(self.g0.w) * other.g1.zzyz * vec4(0.0, -1.0, 1.0, 0.0) + vec4(self.g1.x) * other.g2 + vec4(self.g1.z) * vec4(other.g2.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g2.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g0.x, self.g1.y, self.g0.y, self.g0.y) * vec4(other.g1.x, other.g2.x, other.g1.w, other.g1.z) * vec4(0.0, 1.0, -1.0, 1.0), vec4(self.g1.x) * other.g3 + self.g1 * vec4(other.g3.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

MultiVector motor_multi_vector_outer_product(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g1.y) * other.g2.wwwz * vec4(0.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * other.g2.wwwy * vec4(0.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * other.g2.zzyz * vec4(0.0, 1.0, -1.0, 0.0) + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g2.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g3 + vec4(self.g0.z) * vec4(other.g3.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g3.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g3.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

MultiVector motor_multi_vector_inner_product(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) - vec4(self.g1.x) * other.g2 + vec4(self.g1.y) * vec4(other.g2.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g2.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g2.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g2 + vec4(self.g0.z) * other.g2.wwxy * vec4(0.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g2.zzyx * vec4(0.0, 1.0, -1.0, -1.0) + self.g0.xyyy * other.g2.xxwz * vec4(0.0, -1.0, 1.0, -1.0), vec4(self.g0.x) * other.g3 + vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0 * vec4(other.g3.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

MultiVector motor_multi_vector_left_contraction(Motor self, MultiVector other) {
    return MultiVector(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g2 + self.g0 * vec4(other.g2.x) * vec4(0.0, -1.0, -1.0, -1.0), vec4(self.g0.x) * other.g3 + self.g0 * vec4(other.g3.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

Scalar motor_multi_vector_scalar_product(Motor self, MultiVector other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Rotor motor_rotor_into(Motor self) {
    return Rotor(self.g0);
}

Motor motor_rotor_add(Motor self, Rotor other) {
    return Motor(self.g0 + other.g0, self.g1);
}

Motor motor_rotor_sub(Motor self, Rotor other) {
    return Motor(self.g0 - other.g0, self.g1);
}

Motor motor_rotor_geometric_product(Motor self, Rotor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

Rotor motor_rotor_regressive_product(Motor self, Rotor other) {
    return Rotor(vec4(self.g1.x) * other.g0 + vec4(self.g1.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + self.g1.yxxx * other.g0.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor motor_rotor_outer_product(Motor self, Rotor other) {
    return Motor(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor motor_rotor_inner_product(Motor self, Rotor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g1 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Rotor motor_rotor_left_contraction(Motor self, Rotor other) {
    return Rotor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0));
}

Motor motor_rotor_right_contraction(Motor self, Rotor other) {
    return Motor(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g1 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar motor_rotor_scalar_product(Motor self, Rotor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Point motor_point_outer_product(Motor self, Point other) {
    return Point(vec4(self.g0.x) * other.g0);
}

Plane motor_plane_regressive_product(Motor self, Plane other) {
    return Plane(vec4(self.g1.x) * other.g0);
}

Plane motor_plane_left_contraction(Motor self, Plane other) {
    return Plane(vec4(self.g0.x) * other.g0);
}

Line motor_line_into(Motor self) {
    return Line(vec3(self.g1.y, self.g1.z, self.g1.w), vec3(self.g0.y, self.g0.z, self.g0.w));
}

Motor motor_line_add(Motor self, Line other) {
    return Motor(self.g0 + vec4(other.g0.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), self.g1 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_line_sub(Motor self, Line other) {
    return Motor(self.g0 - vec4(other.g0.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), self.g1 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_line_geometric_product(Motor self, Line other) {
    return Motor(vec4(self.g0.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(-1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(-1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(-1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.y) * vec4(other.g0.x, other.g0.x, other.g0.z, other.g0.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * vec4(other.g0.y, other.g0.z, other.g0.y, other.g0.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * vec4(other.g0.z, other.g0.y, other.g0.x, other.g0.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(0.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g1.x, other.g1.x, other.g1.z, other.g1.y) * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g1.z) * vec4(other.g1.y, other.g1.z, other.g1.y, other.g1.x) * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g1.w) * vec4(other.g1.z, other.g1.y, other.g1.x, other.g1.z) * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g0.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Translator motor_line_right_contraction(Motor self, Line other) {
    return Translator(vec4(self.g0.z) * vec4(other.g1.y) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.y, self.g1.x, self.g1.x, self.g1.x) * vec4(other.g1.x, other.g1.x, other.g1.y, other.g1.z) * vec4(-1.0));
}

Scalar motor_line_scalar_product(Motor self, Line other) {
    return Scalar(0.0 - self.g0.y * other.g1.x - self.g0.z * other.g1.y - self.g0.w * other.g1.z);
}

Translator motor_translator_into(Motor self) {
    return Translator(vec4(self.g0.x, self.g1.y, self.g1.z, self.g1.w));
}

Motor motor_translator_add(Motor self, Translator other) {
    return Motor(self.g0 + vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g1 + other.g0 * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_translator_sub(Motor self, Translator other) {
    return Motor(self.g0 - vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), self.g1 - other.g0 * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_translator_geometric_product(Motor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.x), vec4(self.g0.y) * other.g0.yywz * vec4(1.0, 0.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwzy * vec4(1.0, -1.0, 0.0, 1.0) + vec4(self.g0.w) * other.g0.wzyw * vec4(1.0, 1.0, -1.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g0);
}

Translator motor_translator_regressive_product(Motor self, Translator other) {
    return Translator(vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * other.g0 + self.g0.yxxx * other.g0.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor motor_translator_outer_product(Motor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.x), vec4(self.g0.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g0.yyzw);
}

Motor motor_translator_inner_product(Motor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.x), vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + vec4(self.g1.x, self.g0.x, self.g0.x, self.g0.x) * other.g0);
}

Translator motor_translator_left_contraction(Motor self, Translator other) {
    return Translator(vec4(self.g0.x) * other.g0);
}

Motor motor_translator_right_contraction(Motor self, Translator other) {
    return Motor(self.g0 * vec4(other.g0.x), self.g1 * vec4(other.g0.x));
}

Scalar motor_translator_scalar_product(Motor self, Translator other) {
    return Scalar(self.g0.x * other.g0.x);
}

Motor motor_motor_add(Motor self, Motor other) {
    return Motor(self.g0 + other.g0, self.g1 + other.g1);
}

Motor motor_motor_sub(Motor self, Motor other) {
    return Motor(self.g0 - other.g0, self.g1 - other.g1);
}

Motor motor_motor_geometric_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.y) * other.g0.yxwz * vec4(-1.0, 1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g0.zwxy * vec4(-1.0, -1.0, 1.0, 1.0) + vec4(self.g0.w) * other.g0.wzyx * vec4(-1.0, 1.0, -1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.y) * other.g1.yxwz * vec4(1.0, -1.0, 1.0, -1.0) + vec4(self.g0.z) * other.g1.zwxy * vec4(1.0, -1.0, -1.0, 1.0) + vec4(self.g0.w) * other.g1.wzyx * vec4(1.0, 1.0, -1.0, -1.0) + vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * other.g0.yxwz * vec4(1.0, 1.0, 1.0, -1.0) + vec4(self.g1.z) * other.g0.zwxy * vec4(1.0, -1.0, 1.0, 1.0) + vec4(self.g1.w) * other.g0.wzyx * vec4(1.0, 1.0, -1.0, 1.0));
}

Motor motor_motor_regressive_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g1.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g1.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g1.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g1.x) * other.g0 + vec4(self.g1.y) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.x) * vec4(other.g1.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.x) * other.g1 + self.g1 * vec4(other.g1.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_motor_outer_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + self.g0 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0), vec4(self.g0.x) * other.g1 + vec4(self.g0.z) * vec4(other.g1.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g1.w) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g1.y) * other.g0.yxyy * vec4(1.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * other.g0.zzxz * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * other.g0.wwwx * vec4(1.0, 0.0, 0.0, 1.0) + self.g0.yxxx * other.g1.yxxx * vec4(1.0, 0.0, 0.0, 0.0));
}

Motor motor_motor_inner_product(Motor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + self.g0.yyxx * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + vec4(self.g1.y) * vec4(other.g0.x) * vec4(0.0, 1.0, 0.0, 0.0) + vec4(self.g1.z) * vec4(other.g0.x) * vec4(0.0, 0.0, 1.0, 0.0) + vec4(self.g1.w) * vec4(other.g0.x) * vec4(0.0, 0.0, 0.0, 1.0) + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

Motor motor_motor_left_contraction(Motor self, Motor other) {
    return Motor(vec4(self.g0.x) * other.g0 + vec4(self.g0.z) * vec4(other.g0.z) * vec4(-1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.w) * vec4(-1.0, 0.0, 0.0, 0.0) + self.g0.yxxx * other.g0.yxxx * vec4(-1.0, 0.0, 0.0, 0.0), vec4(self.g0.x) * other.g1 + self.g0 * vec4(other.g1.x) * vec4(0.0, -1.0, -1.0, -1.0));
}

Motor motor_motor_right_contraction(Motor self, Motor other) {
    return Motor(vec4(self.g0.y) * other.g0.yxyy * vec4(-1.0, 1.0, 0.0, 0.0) + vec4(self.g0.z) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.w) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.x) * vec4(1.0, 0.0, 0.0, 0.0), vec4(self.g1.x) * other.g0 * vec4(1.0, -1.0, -1.0, -1.0) + self.g1 * vec4(other.g0.x) * vec4(0.0, 1.0, 1.0, 1.0));
}

Scalar motor_motor_scalar_product(Motor self, Motor other) {
    return Scalar(self.g0.x * other.g0.x - self.g0.y * other.g0.y - self.g0.z * other.g0.z - self.g0.w * other.g0.w);
}

Branch motor_branch_into(Motor self) {
    return Branch(vec3(self.g1.y, self.g1.z, self.g1.w));
}

Motor motor_branch_add(Motor self, Branch other) {
    return Motor(self.g0, self.g1 + vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Motor motor_branch_sub(Motor self, Branch other) {
    return Motor(self.g0, self.g1 - vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z) * vec4(0.0, 1.0, 1.0, 1.0));
}

Translator motor_branch_regressive_product(Motor self, Branch other) {
    return Translator(vec4(self.g0.z) * vec4(other.g0.y) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.w) * vec4(other.g0.z) * vec4(1.0, 0.0, 0.0, 0.0) + vec4(self.g0.y, self.g1.x, self.g1.x, self.g1.x) * vec4(other.g0.x, other.g0.x, other.g0.y, other.g0.z));
}

Branch motor_branch_inner_product(Motor self, Branch other) {
    return Branch(vec3(self.g0.x) * other.g0);
}

Branch motor_branch_left_contraction(Motor self, Branch other) {
    return Branch(vec3(self.g0.x) * other.g0);
}

Scalar motor_squared_magnitude(Motor self) {
    return motor_motor_scalar_product(self, motor_reversal(self));
}

Scalar motor_magnitude(Motor self) {
    return Scalar(sqrt(motor_squared_magnitude(self).g0));
}

Motor motor_signum(Motor self) {
    return motor_scalar_geometric_product(self, Scalar(1.0 / motor_magnitude(self).g0));
}

Motor motor_inverse(Motor self) {
    return motor_scalar_geometric_product(motor_reversal(self), Scalar(1.0 / motor_squared_magnitude(self).g0));
}

Branch branch_zero() {
    return Branch(vec3(0.0));
}

Branch branch_one() {
    return Branch(vec3(0.0));
}

Branch branch_neg(Branch self) {
    return Branch(self.g0 * vec3(-1.0));
}

Branch branch_automorphism(Branch self) {
    return Branch(self.g0);
}

Branch branch_reversal(Branch self) {
    return Branch(self.g0 * vec3(-1.0));
}

Branch branch_conjugation(Branch self) {
    return Branch(self.g0 * vec3(-1.0));
}

Translator branch_scalar_add(Branch self, Scalar other) {
    return Translator(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Translator branch_scalar_sub(Branch self, Scalar other) {
    return Translator(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - vec4(other.g0) * vec4(1.0, 0.0, 0.0, 0.0));
}

Branch branch_scalar_geometric_product(Branch self, Scalar other) {
    return Branch(self.g0 * vec3(other.g0));
}

Branch branch_scalar_outer_product(Branch self, Scalar other) {
    return Branch(self.g0 * vec3(other.g0));
}

Branch branch_scalar_inner_product(Branch self, Scalar other) {
    return Branch(self.g0 * vec3(other.g0));
}

Branch branch_scalar_right_contraction(Branch self, Scalar other) {
    return Branch(self.g0 * vec3(other.g0));
}

MultiVector branch_multi_vector_add(Branch self, MultiVector other) {
    return MultiVector(other.g0, other.g1, other.g2, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g3);
}

MultiVector branch_multi_vector_sub(Branch self, MultiVector other) {
    return MultiVector(vec4(0.0) - other.g0, vec4(0.0) - other.g1, vec4(0.0) - other.g2, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g3);
}

Scalar branch_rotor_regressive_product(Branch self, Rotor other) {
    return Scalar(self.g0.x * other.g0.y + self.g0.y * other.g0.z + self.g0.z * other.g0.w);
}

Branch branch_rotor_inner_product(Branch self, Rotor other) {
    return Branch(self.g0 * vec3(other.g0.x));
}

Branch branch_rotor_right_contraction(Branch self, Rotor other) {
    return Branch(self.g0 * vec3(other.g0.x));
}

Plane branch_point_regressive_product(Branch self, Point other) {
    return Plane(vec4(self.g0.y) * other.g0.zzxz * vec4(-1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * other.g0.wwwx * vec4(-1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * other.g0.yxxx * vec4(-1.0, 1.0, 0.0, 0.0));
}

Line branch_line_add(Branch self, Line other) {
    return Line(self.g0 + other.g0, other.g1);
}

Line branch_line_sub(Branch self, Line other) {
    return Line(self.g0 - other.g0, vec3(0.0) - other.g1);
}

Scalar branch_line_regressive_product(Branch self, Line other) {
    return Scalar(self.g0.x * other.g1.x + self.g0.y * other.g1.y + self.g0.z * other.g1.z);
}

Translator branch_translator_add(Branch self, Translator other) {
    return Translator(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g0);
}

Translator branch_translator_sub(Branch self, Translator other) {
    return Translator(vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g0);
}

Branch branch_translator_geometric_product(Branch self, Translator other) {
    return Branch(self.g0 * vec3(other.g0.x));
}

Branch branch_translator_outer_product(Branch self, Translator other) {
    return Branch(self.g0 * vec3(other.g0.x));
}

Branch branch_translator_inner_product(Branch self, Translator other) {
    return Branch(self.g0 * vec3(other.g0.x));
}

Branch branch_translator_right_contraction(Branch self, Translator other) {
    return Branch(self.g0 * vec3(other.g0.x));
}

Motor branch_motor_add(Branch self, Motor other) {
    return Motor(other.g0, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) + other.g1);
}

Motor branch_motor_sub(Branch self, Motor other) {
    return Motor(vec4(0.0) - other.g0, vec4(self.g0.x, self.g0.x, self.g0.y, self.g0.z) * vec4(0.0, 1.0, 1.0, 1.0) - other.g1);
}

Translator branch_motor_regressive_product(Branch self, Motor other) {
    return Translator(vec4(self.g0.y) * vec4(other.g0.z, other.g0.z, other.g1.x, other.g0.z) * vec4(1.0, 0.0, 1.0, 0.0) + vec4(self.g0.z) * vec4(other.g0.w, other.g0.w, other.g0.w, other.g1.x) * vec4(1.0, 0.0, 0.0, 1.0) + vec4(self.g0.x) * vec4(other.g0.y, other.g1.x, other.g0.x, other.g0.x) * vec4(1.0, 1.0, 0.0, 0.0));
}

Branch branch_motor_inner_product(Branch self, Motor other) {
    return Branch(self.g0 * vec3(other.g0.x));
}

Branch branch_motor_right_contraction(Branch self, Motor other) {
    return Branch(self.g0 * vec3(other.g0.x));
}

Branch branch_branch_add(Branch self, Branch other) {
    return Branch(self.g0 + other.g0);
}

Branch branch_branch_sub(Branch self, Branch other) {
    return Branch(self.g0 - other.g0);
}

Branch branch_scalar_mul(Branch self, Scalar other) {
    return branch_scalar_geometric_product(self, other);
}

Branch branch_scalar_div(Branch self, Scalar other) {
    return branch_scalar_geometric_product(self, scalar_inverse(other));
}

Branch branch_translator_mul(Branch self, Translator other) {
    return branch_translator_geometric_product(self, other);
}

Branch branch_translator_div(Branch self, Translator other) {
    return branch_translator_geometric_product(self, translator_inverse(other));
}

Motor line_line_mul(Line self, Line other) {
    return line_line_geometric_product(self, other);
}

Motor line_line_div(Line self, Line other) {
    return line_line_geometric_product(self, line_inverse(other));
}

Line line_line_transformation(Line self, Line other) {
    return motor_line_into(motor_line_geometric_product(line_line_geometric_product(self, other), line_reversal(self)));
}

Motor line_motor_mul(Line self, Motor other) {
    return line_motor_geometric_product(self, other);
}

Motor line_motor_div(Line self, Motor other) {
    return line_motor_geometric_product(self, motor_inverse(other));
}

Motor line_motor_transformation(Line self, Motor other) {
    return motor_line_geometric_product(line_motor_geometric_product(self, other), line_reversal(self));
}

MultiVector line_multi_vector_mul(Line self, MultiVector other) {
    return line_multi_vector_geometric_product(self, other);
}

MultiVector line_multi_vector_div(Line self, MultiVector other) {
    return line_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector line_multi_vector_transformation(Line self, MultiVector other) {
    return multi_vector_line_geometric_product(line_multi_vector_geometric_product(self, other), line_reversal(self));
}

Motor line_rotor_mul(Line self, Rotor other) {
    return line_rotor_geometric_product(self, other);
}

Motor line_rotor_div(Line self, Rotor other) {
    return line_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor line_rotor_transformation(Line self, Rotor other) {
    return motor_rotor_into(motor_line_geometric_product(line_rotor_geometric_product(self, other), line_reversal(self)));
}

Line line_scalar_mul(Line self, Scalar other) {
    return line_scalar_geometric_product(self, other);
}

Line line_scalar_div(Line self, Scalar other) {
    return line_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar line_scalar_transformation(Line self, Scalar other) {
    return motor_scalar_into(line_line_geometric_product(line_scalar_geometric_product(self, other), line_reversal(self)));
}

Motor motor_line_mul(Motor self, Line other) {
    return motor_line_geometric_product(self, other);
}

Motor motor_line_div(Motor self, Line other) {
    return motor_line_geometric_product(self, line_inverse(other));
}

Line motor_line_transformation(Motor self, Line other) {
    return motor_line_into(motor_motor_geometric_product(motor_line_geometric_product(self, other), motor_reversal(self)));
}

Motor motor_motor_mul(Motor self, Motor other) {
    return motor_motor_geometric_product(self, other);
}

Motor motor_powi(Motor self, int exponent) {
    if(exponent == 0) {
        return motor_one();
    }
    Motor x = (exponent < 0) ? motor_inverse(self) : self;
    Motor y = motor_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = motor_motor_geometric_product(x, y);
        }
        x = motor_motor_geometric_product(x, x);
        n = n >> 1;
    }
    return motor_motor_geometric_product(x, y);
}

Motor motor_motor_div(Motor self, Motor other) {
    return motor_motor_geometric_product(self, motor_inverse(other));
}

Motor motor_motor_transformation(Motor self, Motor other) {
    return motor_motor_geometric_product(motor_motor_geometric_product(self, other), motor_reversal(self));
}

MultiVector motor_multi_vector_mul(Motor self, MultiVector other) {
    return motor_multi_vector_geometric_product(self, other);
}

MultiVector motor_multi_vector_div(Motor self, MultiVector other) {
    return motor_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector motor_multi_vector_transformation(Motor self, MultiVector other) {
    return multi_vector_motor_geometric_product(motor_multi_vector_geometric_product(self, other), motor_reversal(self));
}

Motor motor_rotor_mul(Motor self, Rotor other) {
    return motor_rotor_geometric_product(self, other);
}

Motor motor_rotor_div(Motor self, Rotor other) {
    return motor_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor motor_rotor_transformation(Motor self, Rotor other) {
    return motor_rotor_into(motor_motor_geometric_product(motor_rotor_geometric_product(self, other), motor_reversal(self)));
}

Motor motor_scalar_mul(Motor self, Scalar other) {
    return motor_scalar_geometric_product(self, other);
}

Motor motor_scalar_div(Motor self, Scalar other) {
    return motor_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar motor_scalar_transformation(Motor self, Scalar other) {
    return motor_scalar_into(motor_motor_geometric_product(motor_scalar_geometric_product(self, other), motor_reversal(self)));
}

Motor motor_translator_mul(Motor self, Translator other) {
    return motor_translator_geometric_product(self, other);
}

Motor motor_translator_div(Motor self, Translator other) {
    return motor_translator_geometric_product(self, translator_inverse(other));
}

Translator motor_translator_transformation(Motor self, Translator other) {
    return motor_translator_into(motor_motor_geometric_product(motor_translator_geometric_product(self, other), motor_reversal(self)));
}

MultiVector multi_vector_line_mul(MultiVector self, Line other) {
    return multi_vector_line_geometric_product(self, other);
}

MultiVector multi_vector_line_div(MultiVector self, Line other) {
    return multi_vector_line_geometric_product(self, line_inverse(other));
}

Line multi_vector_line_transformation(MultiVector self, Line other) {
    return multi_vector_line_into(multi_vector_multi_vector_geometric_product(multi_vector_line_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_motor_mul(MultiVector self, Motor other) {
    return multi_vector_motor_geometric_product(self, other);
}

MultiVector multi_vector_motor_div(MultiVector self, Motor other) {
    return multi_vector_motor_geometric_product(self, motor_inverse(other));
}

Motor multi_vector_motor_transformation(MultiVector self, Motor other) {
    return multi_vector_motor_into(multi_vector_multi_vector_geometric_product(multi_vector_motor_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_multi_vector_mul(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_geometric_product(self, other);
}

MultiVector multi_vector_powi(MultiVector self, int exponent) {
    if(exponent == 0) {
        return multi_vector_one();
    }
    MultiVector x = (exponent < 0) ? multi_vector_inverse(self) : self;
    MultiVector y = multi_vector_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = multi_vector_multi_vector_geometric_product(x, y);
        }
        x = multi_vector_multi_vector_geometric_product(x, x);
        n = n >> 1;
    }
    return multi_vector_multi_vector_geometric_product(x, y);
}

MultiVector multi_vector_multi_vector_div(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector multi_vector_multi_vector_transformation(MultiVector self, MultiVector other) {
    return multi_vector_multi_vector_geometric_product(multi_vector_multi_vector_geometric_product(self, other), multi_vector_reversal(self));
}

MultiVector multi_vector_plane_mul(MultiVector self, Plane other) {
    return multi_vector_plane_geometric_product(self, other);
}

MultiVector multi_vector_plane_div(MultiVector self, Plane other) {
    return multi_vector_plane_geometric_product(self, plane_inverse(other));
}

Plane multi_vector_plane_transformation(MultiVector self, Plane other) {
    return multi_vector_plane_into(multi_vector_multi_vector_geometric_product(multi_vector_plane_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_point_mul(MultiVector self, Point other) {
    return multi_vector_point_geometric_product(self, other);
}

MultiVector multi_vector_point_div(MultiVector self, Point other) {
    return multi_vector_point_geometric_product(self, point_inverse(other));
}

Point multi_vector_point_transformation(MultiVector self, Point other) {
    return multi_vector_point_into(multi_vector_multi_vector_geometric_product(multi_vector_point_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_rotor_mul(MultiVector self, Rotor other) {
    return multi_vector_rotor_geometric_product(self, other);
}

MultiVector multi_vector_rotor_div(MultiVector self, Rotor other) {
    return multi_vector_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor multi_vector_rotor_transformation(MultiVector self, Rotor other) {
    return multi_vector_rotor_into(multi_vector_multi_vector_geometric_product(multi_vector_rotor_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_scalar_mul(MultiVector self, Scalar other) {
    return multi_vector_scalar_geometric_product(self, other);
}

MultiVector multi_vector_scalar_div(MultiVector self, Scalar other) {
    return multi_vector_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar multi_vector_scalar_transformation(MultiVector self, Scalar other) {
    return multi_vector_scalar_into(multi_vector_multi_vector_geometric_product(multi_vector_scalar_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector multi_vector_translator_mul(MultiVector self, Translator other) {
    return multi_vector_translator_geometric_product(self, other);
}

MultiVector multi_vector_translator_div(MultiVector self, Translator other) {
    return multi_vector_translator_geometric_product(self, translator_inverse(other));
}

Translator multi_vector_translator_transformation(MultiVector self, Translator other) {
    return multi_vector_translator_into(multi_vector_multi_vector_geometric_product(multi_vector_translator_geometric_product(self, other), multi_vector_reversal(self)));
}

MultiVector plane_multi_vector_mul(Plane self, MultiVector other) {
    return plane_multi_vector_geometric_product(self, other);
}

MultiVector plane_multi_vector_div(Plane self, MultiVector other) {
    return plane_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector plane_multi_vector_transformation(Plane self, MultiVector other) {
    return multi_vector_plane_geometric_product(plane_multi_vector_geometric_product(self, other), plane_reversal(self));
}

Plane plane_scalar_mul(Plane self, Scalar other) {
    return plane_scalar_geometric_product(self, other);
}

Plane plane_scalar_div(Plane self, Scalar other) {
    return plane_scalar_geometric_product(self, scalar_inverse(other));
}

MultiVector point_multi_vector_mul(Point self, MultiVector other) {
    return point_multi_vector_geometric_product(self, other);
}

MultiVector point_multi_vector_div(Point self, MultiVector other) {
    return point_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector point_multi_vector_transformation(Point self, MultiVector other) {
    return multi_vector_point_geometric_product(point_multi_vector_geometric_product(self, other), point_reversal(self));
}

Translator point_point_mul(Point self, Point other) {
    return point_point_geometric_product(self, other);
}

Translator point_point_div(Point self, Point other) {
    return point_point_geometric_product(self, point_inverse(other));
}

Point point_point_transformation(Point self, Point other) {
    return translator_point_geometric_product(point_point_geometric_product(self, other), point_reversal(self));
}

Point point_scalar_mul(Point self, Scalar other) {
    return point_scalar_geometric_product(self, other);
}

Point point_scalar_div(Point self, Scalar other) {
    return point_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar point_scalar_transformation(Point self, Scalar other) {
    return translator_scalar_into(point_point_geometric_product(point_scalar_geometric_product(self, other), point_reversal(self)));
}

Point point_translator_mul(Point self, Translator other) {
    return point_translator_geometric_product(self, other);
}

Point point_translator_div(Point self, Translator other) {
    return point_translator_geometric_product(self, translator_inverse(other));
}

Translator point_translator_transformation(Point self, Translator other) {
    return point_point_geometric_product(point_translator_geometric_product(self, other), point_reversal(self));
}

Motor rotor_line_mul(Rotor self, Line other) {
    return rotor_line_geometric_product(self, other);
}

Motor rotor_line_div(Rotor self, Line other) {
    return rotor_line_geometric_product(self, line_inverse(other));
}

Line rotor_line_transformation(Rotor self, Line other) {
    return motor_line_into(motor_rotor_geometric_product(rotor_line_geometric_product(self, other), rotor_reversal(self)));
}

Motor rotor_motor_mul(Rotor self, Motor other) {
    return rotor_motor_geometric_product(self, other);
}

Motor rotor_motor_div(Rotor self, Motor other) {
    return rotor_motor_geometric_product(self, motor_inverse(other));
}

Motor rotor_motor_transformation(Rotor self, Motor other) {
    return motor_rotor_geometric_product(rotor_motor_geometric_product(self, other), rotor_reversal(self));
}

MultiVector rotor_multi_vector_mul(Rotor self, MultiVector other) {
    return rotor_multi_vector_geometric_product(self, other);
}

MultiVector rotor_multi_vector_div(Rotor self, MultiVector other) {
    return rotor_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector rotor_multi_vector_transformation(Rotor self, MultiVector other) {
    return multi_vector_rotor_geometric_product(rotor_multi_vector_geometric_product(self, other), rotor_reversal(self));
}

Rotor rotor_rotor_mul(Rotor self, Rotor other) {
    return rotor_rotor_geometric_product(self, other);
}

Rotor rotor_powi(Rotor self, int exponent) {
    if(exponent == 0) {
        return rotor_one();
    }
    Rotor x = (exponent < 0) ? rotor_inverse(self) : self;
    Rotor y = rotor_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = rotor_rotor_geometric_product(x, y);
        }
        x = rotor_rotor_geometric_product(x, x);
        n = n >> 1;
    }
    return rotor_rotor_geometric_product(x, y);
}

Rotor rotor_rotor_div(Rotor self, Rotor other) {
    return rotor_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor rotor_rotor_transformation(Rotor self, Rotor other) {
    return rotor_rotor_geometric_product(rotor_rotor_geometric_product(self, other), rotor_reversal(self));
}

Rotor rotor_scalar_mul(Rotor self, Scalar other) {
    return rotor_scalar_geometric_product(self, other);
}

Rotor rotor_scalar_div(Rotor self, Scalar other) {
    return rotor_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar rotor_scalar_transformation(Rotor self, Scalar other) {
    return rotor_scalar_into(rotor_rotor_geometric_product(rotor_scalar_geometric_product(self, other), rotor_reversal(self)));
}

Motor rotor_translator_mul(Rotor self, Translator other) {
    return rotor_translator_geometric_product(self, other);
}

Motor rotor_translator_div(Rotor self, Translator other) {
    return rotor_translator_geometric_product(self, translator_inverse(other));
}

Translator rotor_translator_transformation(Rotor self, Translator other) {
    return motor_translator_into(motor_rotor_geometric_product(rotor_translator_geometric_product(self, other), rotor_reversal(self)));
}

Branch scalar_branch_mul(Scalar self, Branch other) {
    return scalar_branch_geometric_product(self, other);
}

Branch scalar_branch_transformation(Scalar self, Branch other) {
    return branch_scalar_geometric_product(scalar_branch_geometric_product(self, other), scalar_reversal(self));
}

Line scalar_line_mul(Scalar self, Line other) {
    return scalar_line_geometric_product(self, other);
}

Line scalar_line_div(Scalar self, Line other) {
    return scalar_line_geometric_product(self, line_inverse(other));
}

Line scalar_line_transformation(Scalar self, Line other) {
    return line_scalar_geometric_product(scalar_line_geometric_product(self, other), scalar_reversal(self));
}

Motor scalar_motor_mul(Scalar self, Motor other) {
    return scalar_motor_geometric_product(self, other);
}

Motor scalar_motor_div(Scalar self, Motor other) {
    return scalar_motor_geometric_product(self, motor_inverse(other));
}

Motor scalar_motor_transformation(Scalar self, Motor other) {
    return motor_scalar_geometric_product(scalar_motor_geometric_product(self, other), scalar_reversal(self));
}

MultiVector scalar_multi_vector_mul(Scalar self, MultiVector other) {
    return scalar_multi_vector_geometric_product(self, other);
}

MultiVector scalar_multi_vector_div(Scalar self, MultiVector other) {
    return scalar_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector scalar_multi_vector_transformation(Scalar self, MultiVector other) {
    return multi_vector_scalar_geometric_product(scalar_multi_vector_geometric_product(self, other), scalar_reversal(self));
}

Plane scalar_plane_mul(Scalar self, Plane other) {
    return scalar_plane_geometric_product(self, other);
}

Plane scalar_plane_div(Scalar self, Plane other) {
    return scalar_plane_geometric_product(self, plane_inverse(other));
}

Plane scalar_plane_transformation(Scalar self, Plane other) {
    return plane_scalar_geometric_product(scalar_plane_geometric_product(self, other), scalar_reversal(self));
}

Point scalar_point_mul(Scalar self, Point other) {
    return scalar_point_geometric_product(self, other);
}

Point scalar_point_div(Scalar self, Point other) {
    return scalar_point_geometric_product(self, point_inverse(other));
}

Point scalar_point_transformation(Scalar self, Point other) {
    return point_scalar_geometric_product(scalar_point_geometric_product(self, other), scalar_reversal(self));
}

Rotor scalar_rotor_mul(Scalar self, Rotor other) {
    return scalar_rotor_geometric_product(self, other);
}

Rotor scalar_rotor_div(Scalar self, Rotor other) {
    return scalar_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor scalar_rotor_transformation(Scalar self, Rotor other) {
    return rotor_scalar_geometric_product(scalar_rotor_geometric_product(self, other), scalar_reversal(self));
}

Scalar scalar_scalar_mul(Scalar self, Scalar other) {
    return scalar_scalar_geometric_product(self, other);
}

Scalar scalar_powi(Scalar self, int exponent) {
    if(exponent == 0) {
        return scalar_one();
    }
    Scalar x = (exponent < 0) ? scalar_inverse(self) : self;
    Scalar y = scalar_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = scalar_scalar_geometric_product(x, y);
        }
        x = scalar_scalar_geometric_product(x, x);
        n = n >> 1;
    }
    return scalar_scalar_geometric_product(x, y);
}

Scalar scalar_scalar_div(Scalar self, Scalar other) {
    return scalar_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar scalar_scalar_transformation(Scalar self, Scalar other) {
    return scalar_scalar_geometric_product(scalar_scalar_geometric_product(self, other), scalar_reversal(self));
}

Translator scalar_translator_mul(Scalar self, Translator other) {
    return scalar_translator_geometric_product(self, other);
}

Translator scalar_translator_div(Scalar self, Translator other) {
    return scalar_translator_geometric_product(self, translator_inverse(other));
}

Translator scalar_translator_transformation(Scalar self, Translator other) {
    return translator_scalar_geometric_product(scalar_translator_geometric_product(self, other), scalar_reversal(self));
}

Branch translator_branch_mul(Translator self, Branch other) {
    return translator_branch_geometric_product(self, other);
}

Branch translator_branch_transformation(Translator self, Branch other) {
    return branch_translator_geometric_product(translator_branch_geometric_product(self, other), translator_reversal(self));
}

Motor translator_motor_mul(Translator self, Motor other) {
    return translator_motor_geometric_product(self, other);
}

Motor translator_motor_div(Translator self, Motor other) {
    return translator_motor_geometric_product(self, motor_inverse(other));
}

Motor translator_motor_transformation(Translator self, Motor other) {
    return motor_translator_geometric_product(translator_motor_geometric_product(self, other), translator_reversal(self));
}

MultiVector translator_multi_vector_mul(Translator self, MultiVector other) {
    return translator_multi_vector_geometric_product(self, other);
}

MultiVector translator_multi_vector_div(Translator self, MultiVector other) {
    return translator_multi_vector_geometric_product(self, multi_vector_inverse(other));
}

MultiVector translator_multi_vector_transformation(Translator self, MultiVector other) {
    return multi_vector_translator_geometric_product(translator_multi_vector_geometric_product(self, other), translator_reversal(self));
}

Point translator_point_mul(Translator self, Point other) {
    return translator_point_geometric_product(self, other);
}

Point translator_point_div(Translator self, Point other) {
    return translator_point_geometric_product(self, point_inverse(other));
}

Point translator_point_transformation(Translator self, Point other) {
    return point_translator_geometric_product(translator_point_geometric_product(self, other), translator_reversal(self));
}

Motor translator_rotor_mul(Translator self, Rotor other) {
    return translator_rotor_geometric_product(self, other);
}

Motor translator_rotor_div(Translator self, Rotor other) {
    return translator_rotor_geometric_product(self, rotor_inverse(other));
}

Rotor translator_rotor_transformation(Translator self, Rotor other) {
    return motor_rotor_into(motor_translator_geometric_product(translator_rotor_geometric_product(self, other), translator_reversal(self)));
}

Translator translator_scalar_mul(Translator self, Scalar other) {
    return translator_scalar_geometric_product(self, other);
}

Translator translator_scalar_div(Translator self, Scalar other) {
    return translator_scalar_geometric_product(self, scalar_inverse(other));
}

Scalar translator_scalar_transformation(Translator self, Scalar other) {
    return translator_scalar_into(translator_translator_geometric_product(translator_scalar_geometric_product(self, other), translator_reversal(self)));
}

Translator translator_translator_mul(Translator self, Translator other) {
    return translator_translator_geometric_product(self, other);
}

Translator translator_powi(Translator self, int exponent) {
    if(exponent == 0) {
        return translator_one();
    }
    Translator x = (exponent < 0) ? translator_inverse(self) : self;
    Translator y = translator_one();
    int n = abs(exponent);
    while(1 < n) {
        if((n & 1) == 1) {
            y = translator_translator_geometric_product(x, y);
        }
        x = translator_translator_geometric_product(x, x);
        n = n >> 1;
    }
    return translator_translator_geometric_product(x, y);
}

Translator translator_translator_div(Translator self, Translator other) {
    return translator_translator_geometric_product(self, translator_inverse(other));
}

Translator translator_translator_transformation(Translator self, Translator other) {
    return translator_translator_geometric_product(translator_translator_geometric_product(self, other), translator_reversal(self));
}

