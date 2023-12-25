#!/usr/bin/env python3

import sympy as sym
ptx, pty, ptz, vtx, vty, vtz = sym.symbols('ptx,pty,ptz,vtx,vty,vtz')
r_0 = sym.symbols('r_0')
eq0 = sym.Eq(346929738756520 + r_0*6, ptx + r_0*vtx)
eq1 = sym.Eq(180308062329517 + r_0*-5, pty + r_0*vty)
eq2 = sym.Eq(348158644025623 + r_0*-22, ptz + r_0*vtz)
r_1 = sym.symbols('r_1')
eq3 = sym.Eq(254810664927620 + r_1*-144, ptx + r_1*vtx)
eq4 = sym.Eq(353739895010289 + r_1*403, pty + r_1*vty)
eq5 = sym.Eq(244141919277765 + r_1*-76, ptz + r_1*vtz)
r_2 = sym.symbols('r_2')
eq6 = sym.Eq(295870756794909 + r_2*-23, ptx + r_2*vtx)
eq7 = sym.Eq(404627177923603 + r_2*-185, pty + r_2*vty)
eq8 = sym.Eq(198720163538165 + r_2*145, ptz + r_2*vtz)
result = sym.solve([eq0,eq1,eq2,eq3,eq4,eq5,eq6,eq7,eq8],(ptx, pty, ptz, vtx, vty, vtz, r_0,r_1,r_2))
print(result)
print(type(result))
