import graph;

size(600,400,IgnoreAspect);

pair unpair(int i) {
  int j = 0;
  int r = i;
  while (r >= j) {
    r -= j;
    j += 1;
  }
  j -= 1;
  return (j - r, r);
}

real[] x={};
real[] y1={};
real[] y2={};
for (int i = 0; i < 500; ++i) {
  x.push(i);
  y1.push(unpair(i).x);
  y2.push(unpair(i).y);
}
real[] z=sqrt(2x);

draw(graph(x,y1),red,"first element");
draw(graph(x,y2),blue,"second element");
draw(graph(x,z),black,"$\sqrt{2x}$");

xaxis(BottomTop,LeftTicks);
yaxis(LeftRight, RightTicks);
add(legend(),point(NW),(25,-25),UnFill);
