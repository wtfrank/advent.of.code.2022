#!/usr/bin/env python3

# 368886062272691 + r_297*-74 = ptz + r_297*vtz
# 266007888635046 + r_298*35 = ptz + r_298*vtz
# 390190548172323 + r_299*-138 = ptz + r_299*vtz

# 283957679948759 + r_297*53 = ptx + r_297*vtx
# 183838154793909 + r_297*47 = pty + r_297*vty
# 328390183181243 + r_298*-31 = ptx + r_298*vtx
# 361677503224065 + r_298*-141 = pty + r_298*vty
# 289605297584183 + r_299*30 = ptx + r_299*vtx
# 215606437887927 + r_299*61 = pty + r_299*vty


import sympy as sym
r_297, r_298, r_299, ptx, pty, ptz, vtx, vty, vtz = sym.symbols('r_297,r_298,r_299,ptx,pty,ptz,vtx,vty,vtz')
eq891 = sym.Eq(283957679948759 + r_297*53, ptx + r_297*vtx)
eq892 = sym.Eq(183838154793909 + r_297*47, pty + r_297*vty)
eq893 = sym.Eq(368886062272691 + r_297*-74, ptz + r_297*vtz)
eq894 = sym.Eq(328390183181243 + r_298*-31, ptx + r_298*vtx)
eq895 = sym.Eq(361677503224065 + r_298*-141, pty + r_298*vty)
eq896 = sym.Eq(266007888635046 + r_298*35, ptz + r_298*vtz)
eq897 = sym.Eq(289605297584183 + r_299*30, ptx + r_299*vtx)
eq898 = sym.Eq(215606437887927 + r_299*61, pty + r_299*vty)
eq899 = sym.Eq(390190548172323 + r_299*-138, ptz + r_299*vtz)
 
result = sym.solve([eq891, eq892, eq893, eq894, eq895, eq896, eq897, eq898, eq899],(r_297, r_298, r_299, ptx, pty, ptz, vtx, vty, vtz))
print(result)
