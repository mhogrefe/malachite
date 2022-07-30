import graph;

size(600,400,IgnoreAspect);

int gcd(int a, int b) {
  while (b != 0) {
    int t = b;
    b = a % b;
    a = t;
  }
  return a;
}

int jacobsthal(int n) {
  int previous = 0;
  int largest_gap = 0;
  for (int i = 0; i <= 2 * n; ++i) {
    if (gcd(i, n) == 1) {
      int gap = i - previous;
      if (gap > largest_gap) {
        largest_gap = gap;
      }
      previous = i;
    }
  }
  return largest_gap;
}

scale(Linear,Log);
real[] x={};
real[] y1={};
real[] y2={};
for (int i = 1; i < 500; ++i) {
  x.push(i);
  y1.push(jacobsthal(i)/i);
  if (i < 6) {
    y2.push(1);
  } else if (i < 30) {
    y2.push(2/3);
  } else if (i < 210) {
    y2.push(4/15);
  } else if (i < 2310) {
    y2.push(8/105);
  }
}

draw(graph(x,y1),black,"$f(n)$");
draw(graph(x,y2),red,"$h(n)$");

xaxis(BottomTop,LeftTicks);
yaxis(LeftRight, RightTicks);
add(legend(),point(NW),(50,-25),UnFill);
