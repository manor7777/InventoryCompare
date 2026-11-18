# Impossible-shape prints (Bambu Lab P1S)

| File | What it is | Size |
|---|---|---|
| `penrose_triangle.stl` | Penrose "impossible triangle". Looks closed only when viewed down its corner diagonal | 90 × 90 × 75 mm |
| `infinite_twist_ring.stl` | Ring with a triangular cross-section and a 120° twist, so it has one endless face and one edge | 110 × 113 × 26 mm |

`preview.png` shows the triangle from the illusion angle, the triangle from the side, and the ring.

## Slicer settings (Bambu Studio, PLA)
- **Penrose triangle:** print it as it loads, with the two long bars flat on the plate. No supports needed (the cut overhangs are 45°). 0.2 mm layers, 15% infill, 3 walls.
- **Twist ring:** lay it flat and turn on **tree supports (auto)**, because the underside curves. 0.12–0.16 mm layers make the twist look smoother.
- **To see the illusion:** close one eye and look at the triangle down the diagonal from the corner where the tall post is. A photo from about 1 m away with zoom gives the best result.

Regenerate or resize: `pip install trimesh manifold3d numpy && python3 generate.py` (edit `L`, `w`, `R`, `r` at the top of each section).

## Amber star gem (original design)
`amber_star_gem.stl` is an 8-point faceted star, 80 × 80 × 20 mm, with twisted crown facets. Regenerate it with `python3 generate_star.py`.
- Print it flat side down with no supports. The girdle overhang is below 45°.
- Use translucent amber or orange PLA or PETG. Set 0.12 mm layers and **100% infill** (or 0 infill with 2 walls) so light passes through it like a gem. Ironing on the top surface is optional.
