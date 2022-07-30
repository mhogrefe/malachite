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

//scale(Linear,Log);
real[] x={};
real[] y={};
for (int i = 1; i < 500; ++i) {
  x.push(i);
  y.push(jacobsthal(i));
}

draw(graph(x,y),black,"$g(n)$");

xaxis(BottomTop,LeftTicks);
yaxis(LeftRight, RightTicks);
add(legend(),point(NW),(25,-25),UnFill);
