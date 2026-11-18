"""Original faceted star gem ('amber star') for translucent filament. Units: mm. Prints flat, no supports."""
import numpy as np, trimesh

N = 8                 # star points
R_OUT, R_IN = 40.0, 22.0
TWIST = np.pi / (2*N) # crown facets twisted a quarter step for a cut-gem look

def star_ring(scale, z, rot=0.0):
    pts = []
    for i in range(2*N):
        a = rot + np.pi*i/N
        r = (R_OUT if i % 2 == 0 else R_IN) * scale
        pts.append([r*np.cos(a), r*np.sin(a), z])
    return pts

# bottom -> girdle -> crown shoulder -> crown table ring -> apex
rings = [
    star_ring(0.90, 0.0),          # flat base on the bed
    star_ring(1.00, 4.0),          # girdle (outward slope < 45 deg)
    star_ring(0.80, 9.0, TWIST),   # shoulder facets
    star_ring(0.42, 15.0, 2*TWIST) # table ring
]
apex_z = 20.0
M = 2*N
V = [p for r in rings for p in r] + [[0, 0, 0.0], [0, 0, apex_z]]
bot, top = len(V)-2, len(V)-1
F = []
for k in range(len(rings)-1):
    for i in range(M):
        a, b = k*M+i, k*M+(i+1) % M
        c, d = (k+1)*M+i, (k+1)*M+(i+1) % M
        F += [[a, b, d], [a, d, c]]
for i in range(M):
    F.append([bot, (i+1) % M, i])                       # base fan
    t = (len(rings)-1)*M
    F.append([t+i, t+(i+1) % M, top])                   # crown apex fan
m = trimesh.Trimesh(np.array(V), np.array(F), process=True)
trimesh.repair.fix_normals(m)
m.export("amber_star_gem.stl")
print("watertight:", m.is_watertight, "size mm:", np.round(m.extents, 1), "vol cm3:", round(m.volume/1000, 1))
