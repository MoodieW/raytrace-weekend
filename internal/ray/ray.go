package ray

import (
	"raytracer_weekend/internal/vec"
)

type Ray struct {
	origin vec.Vec3
	dir    vec.Vec3
}

func NewRay(origin vec.Vec3, direction vec.Vec3) Ray {
	return Ray{origin, direction}
}

func (r Ray) Origin() vec.Vec3 {
	return r.origin
}

func (r Ray) Direction() vec.Vec3 {
	return r.dir
}

func (r Ray) At(t float64) vec.Vec3 {
	return vec.Add(r.origin, vec.MulScalar(t, r.dir))
}
