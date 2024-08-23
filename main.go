package main

import (
	"fmt"
	"image"
	"image/color"
	"image/png"
	"os"
	"raytracer_weekend/internal/ray"
	"raytracer_weekend/internal/vec"
)

type Image struct {
	Width  int
	Hieght int
}

func main() {

	img := Image{256, 256}
	fmt.Printf("P3\n%d %d\n255\n", img.Width, img.Hieght)

	rectangle := image.Rect(0, 0, img.Width, img.Hieght)
	imag := image.NewRGBA(rectangle)

	for i := 0; i <= img.Hieght; i++ {
		fmt.Fprintf(os.Stderr, "\rScanlines remaining: %d ", img.Hieght-i)
		os.Stderr.Sync()
		for x := 0; x <= img.Width; x++ {

			r := float64(i) / float64(img.Width-1)
			g := float64(i) / float64(img.Width-1)
			b := 0.0

			ir := uint8(255.999 * r)
			ig := uint8(255.999 * g)
			ib := uint8(255.999 * b)
			color := color.RGBA{ir, ig, ib, 255}
			imag.Set(i, x, color)
		}
	}

	origin := vec.NewVec3(0, 0, 0)
	direction := vec.NewVec3(1, 1, 1)
	r := ray.NewRay(origin, direction)
	point := r.At(2.0)
	fmt.Printf("Ray at t=2.0: %v\n", point)

	f, _ := os.Create("image.png")
	png.Encode(f, imag)
	f.Close()

	fmt.Println("Image created yo")
}
