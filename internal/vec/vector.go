package vec

import (
	"fmt"
	"math"
)

type Vec3 struct {
	E [3]float64
}

func NewVec3(e0, e1, e2 float64) Vec3 {
	return Vec3{E: [3]float64{e0, e1, e2}}
}

func (v Vec3) X() float64 { return v.E[0] }
func (v Vec3) Y() float64 { return v.E[1] }
func (v Vec3) Z() float64 { return v.E[2] }

func (v Vec3) Negate() Vec3 {
	return NewVec3(-v.E[0], -v.E[1], -v.E[2])
}

func (v *Vec3) Get(i int) float64 {
	return v.E[i]
}

func (v *Vec3) Set(i int, value float64) {
	v.E[i] = value
}

func (v *Vec3) AddAssign(other Vec3) {
	v.E[0] += other.E[0]
	v.E[1] += other.E[1]
	v.E[2] += other.E[2]
}

func (v *Vec3) MulAssing(t float64) {
	v.E[0] *= t
	v.E[1] *= t
	v.E[2] *= t

}

func (v *Vec3) DivAssign(t float64) {
	v.MulAssing(1 / t)
}

func (v *Vec3) Length() float64 {
	return math.Sqrt(v.LengthSqared())
}

func (v *Vec3) LengthSqared() float64 {
	return v.E[0]*v.E[0] + v.E[1]*v.E[1] + v.E[2]*v.E[2]
}

func (v *Vec3) String() string {
	return fmt.Sprintf("%v %v %v", v.E[0], v.E[1], v.E[2])
}

func Add(u, v Vec3) Vec3 {
	return NewVec3(u.E[0]+v.E[0], u.E[1]+v.E[1], u.E[2]+v.E[2])
}

func Sub(u, v Vec3) Vec3 {
	return NewVec3(u.E[0]-v.E[0], u.E[1]-v.E[1], u.E[2]-v.E[2])
}

func Mul(u, v Vec3) Vec3 {
	return NewVec3(u.E[0]*v.E[0], u.E[1]*v.E[1], u.E[2]*v.E[2])
}

func MulScalar(t float64, v Vec3) Vec3 {
	return NewVec3(t*v.E[0], t*v.E[1], t*v.E[2])
}

func DivScalar(v Vec3, t float64) Vec3 {
	return MulScalar(1/t, v)
}

func Dot(u, v Vec3) float64 {
	return u.E[0]*v.E[0] + u.E[1]*v.E[1] + u.E[2]*v.E[2]
}

func Cross(u, v Vec3) Vec3 {
	return NewVec3(u.E[1]*v.E[2]-u.E[2]*v.E[1],
		u.E[2]*v.E[0]-u.E[0]*v.E[2],
		u.E[0]*v.E[1]-u.E[1]*v.E[0])
}

func UnitVector(v Vec3) Vec3 {
	return DivScalar(v, v.Length())
}

// Point3 is just an alias for Vec3
type Point3 = Vec3
