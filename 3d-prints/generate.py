"""Generate printable 'impossible' shapes for a Bambu Lab P1S (256mm bed). Units: mm."""
import numpy as np, trimesh
from manifold3d import Manifold, Mesh

def box(x0, x1, y0, y1, z0, z1):
    return Manifold.cube([x1-x0, y1-y0, z1-z0]).translate([x0, y0, z0])

def to_trimesh(m):
    mm = m.to_mesh()
    return trimesh.Trimesh(vertices=mm.vert_properties[:, :3], faces=mm.tri_verts)

# ---------- 1. Penrose triangle (looks impossible from the isometric corner view) ----------
L, w = 90.0, 15.0
bar1 = box(0, L, 0, w, 0, w)            # along +x, on the bed
bar2 = box(L-w, L, 0, L, 0, w)          # along +y, on the bed
bar3 = box(L-w, L, L-w, L, 0, L)        # vertical, its top lines up with bar1's start along (1,1,1)
# Cut away everything in bar3 that sits in front of bar1 when viewed along (1,1,1)
d = np.array([1, 1, 1]) / np.sqrt(3)
bm = bar1.to_mesh()
pts = bm.vert_properties[:, :3]
shadow = Manifold.hull_points(np.vstack([pts, pts + d * 3 * L]))
bar3_cut = bar3 - shadow
penrose = bar1 + bar2 + bar3_cut
pt = to_trimesh(penrose)
pt.export("penrose_triangle.stl")

# ---------- 2. Infinite twisted ring (triangle-section torus with 1/3 twist: ONE endless face & edge) ----------
R, r, N = 45.0, 13.0, 360       # ring radius, triangle circumradius, segments
verts, faces = [], []
for i in range(N):
    u = 2*np.pi*i/N
    tw = u/3                     # 120 deg twist over one loop -> faces join into one
    for k in range(3):
        a = tw + 2*np.pi*k/3
        rr, zz = r*np.cos(a), r*np.sin(a)
        verts.append([(R+rr)*np.cos(u), (R+rr)*np.sin(u), zz])
for i in range(N):
    j = (i+1) % N
    shift = 1 if j == 0 else 0   # after a full loop the triangle has rotated one vertex
    for k in range(3):
        a, b = i*3+k, i*3+(k+1) % 3
        c, e = j*3+(k+shift) % 3, j*3+(k+1+shift) % 3
        faces += [[a, c, b], [b, c, e]]
ring = trimesh.Trimesh(np.array(verts), np.array(faces))
if ring.volume < 0: ring.invert()
ring.apply_translation([0, 0, -ring.bounds[0][2]])
ring.export("infinite_twist_ring.stl")

for name, m in [("penrose_triangle", pt), ("infinite_twist_ring", ring)]:
    print(name, "watertight:", m.is_watertight, "size mm:", np.round(m.extents, 1), "vol cm3:", round(m.volume/1000, 1))
