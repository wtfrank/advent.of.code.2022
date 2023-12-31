#!/usr/bin/env python3
import sympy as sym

def solve():
  ptx, pty, ptz, vtx, vty, vtz = sym.symbols('ptx,pty,ptz,vtx,vty,vtz')
  r_0 = sym.symbols('r_0')
  eq0 = sym.Eq(19 + r_0*-2, ptx + r_0*vtx)
  eq1 = sym.Eq(13 + r_0*1, pty + r_0*vty)
  eq2 = sym.Eq(30 + r_0*-2, ptz + r_0*vtz)
  r_1 = sym.symbols('r_1')
  eq3 = sym.Eq(18 + r_1*-1, ptx + r_1*vtx)
  eq4 = sym.Eq(19 + r_1*-1, pty + r_1*vty)
  eq5 = sym.Eq(22 + r_1*-2, ptz + r_1*vtz)
  r_2 = sym.symbols('r_2')
  eq6 = sym.Eq(20 + r_2*-2, ptx + r_2*vtx)
  eq7 = sym.Eq(25 + r_2*-2, pty + r_2*vty)
  eq8 = sym.Eq(34 + r_2*-4, ptz + r_2*vtz)
  #result = sym.solve([eq0,eq1,eq2,eq3,eq4,eq5,eq6,eq7,eq8],(ptx, pty, ptz, vtx, vty, vtz, r_0,r_1,r_2))

  # forcing linear solver gives expected error about non-linear terms
  #
  # result = sym.linsolve([eq0,eq1,eq2,eq3,eq4,eq5,eq6,eq7,eq8],(ptx, pty, ptz, vtx, vty, vtz, r_0,r_1,r_2))
  # sympy.solvers.solveset.NonlinearError: 
  # nonlinear cross-term: r_0*vtx

  system = [eq0,eq1,eq2,eq3,eq4,eq5,eq6,eq7,eq8]
  print("zero dimensional : ", sym.polys.polytools.is_zero_dimensional(system))
  #result = sym.nonlinsolve([eq0,eq1,eq2,eq3,eq4,eq5,eq6,eq7,eq8],(ptx, pty, ptz, vtx, vty, vtz, r_0,r_1,r_2))


  gbasis = sym.groebner(system, ptx, pty, ptz, vtx, vty, vtz, r_0,r_1,r_2)
  print("groebner basis:", gbasis)
  print("system:", system)

  result = sym.solvers.polysys.solve_poly_system([eq0,eq1,eq2,eq3,eq4,eq5,eq6,eq7,eq8],(ptx, pty, ptz, vtx, vty, vtz, r_0,r_1,r_2))
  return result

print(solve())
