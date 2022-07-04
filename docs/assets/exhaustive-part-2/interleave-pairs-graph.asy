import graph;

size(600,400,IgnoreAspect);

pair unpair(int i) {
  int x = 0;
  int y = 0;
  bool on_x = false;
  int out_mask = 1;
  while (i != 0) {
    if (i % 2 != 0) {
      if (on_x) {
        x += out_mask;
      } else {
        y += out_mask;
      }
    }
    if (on_x) {
      on_x = false;
      out_mask *= 2;
    } else {
      on_x = true;
    }
    i #= 2;
  }
  return (x, y);
}

real[] x={};
real[] y1={};
real[] y2={};
for (int i = 0; i < 10000; ++i) {
  x.push(i);
  y1.push(unpair(i).x);
  y2.push(unpair(i).y);
}
real[] z=sqrt(3*x);

draw(graph(x,y1),red,"first element");
draw(graph(x,y2),blue,"second element");
draw(graph(x,z),black,"$\sqrt{3x}$");

xaxis(BottomTop,LeftTicks);
yaxis(LeftRight, RightTicks);
add(legend(),point(NW),(25,-25),UnFill);
