import graph;

size(600,400,IgnoreAspect);

triple untriple(int i) {
  int x = 0;
  int y = 0;
  int z = 0;
  int j = 2;
  int out_mask = 1;
  while (i != 0) {
    if (i % 2 != 0) {
      if (j == 0) {
        x += out_mask;
      } else if (j == 1) {
        y += out_mask;
      } else {
        z += out_mask;
      }
    }
    if (j == 0) {
      j = 2;
      out_mask *= 2;
    } else if (j == 1) {
      j = 0;
    } else {
      j = 1;
    }
    i #= 2;
  }
  return (x, y, z);
}

real[] x={};
real[] y1={};
real[] y2={};
real[] y3={};
for (int i = 0; i < 10000; ++i) {
  x.push(i);
  y1.push(untriple(i).x);
  y2.push(untriple(i).y);
  y3.push(untriple(i).z);
}
real[] z=(7*x)^(1/3);

draw(graph(x,y1),red,"first element");
draw(graph(x,y2),blue,"second element");
draw(graph(x,y3),green,"third element");
draw(graph(x,z),black,"$\sqrt[3]{7x}$");

xaxis(BottomTop,LeftTicks);
yaxis(LeftRight, RightTicks);
add(legend(),point(NW),(25,-25),UnFill);
