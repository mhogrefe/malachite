import graph;
size(15cm);
pen small=fontcommand("\scriptsize");

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

string gap_function(int d) {
  int n = jacobsthal(d);
  int g = gcd(n, d);
  n = n#g;
  d = d#g;
  if (d == 1) {
    return string(n);
  } else {
    return string(n) + "/" + string(d);
  }
}

void draw_rationals(int d, real offset) {
  guide gaxis=(0,offset)--(20,offset);
  draw(gaxis);
  for(int i = 0; i < 3; ++i){
    tick(relpoint(gaxis,i/2),-plain.I*reldir(gaxis,i/2),ticksize*2);
  }
  for(int i = 0; i < 20; ++i){
    tick(relpoint(gaxis,i/20),-plain.I*reldir(gaxis,i/20),ticksize);
  }
  label("$0$",relpoint(gaxis,0),-3*plain.I*reldir(gaxis,0));
  label("$1$",relpoint(gaxis,0.5),-3*plain.I*reldir(gaxis,0.5));
  label("$2$",relpoint(gaxis,1),-3*plain.I*reldir(gaxis,1));
  
  for (int i= 0; i <= 2 * d; ++i) {
    if (gcd(i, d) == 1) {
      dot((i*10/d, offset), red);
      if (d != 1) {
        label("$" + string(i) + "/" + string(d) + "$",relpoint(gaxis,i/(2*d)),-2*plain.I*reldir(gaxis,0.5), red+small);
      }
    }
  }
  
  label("$f(" + string(d) + ") = " + gap_function(d) + "$",(24.5, offset), align=LeftSide);
}

real offset = 0;
for (int d = 1; d <= 10; ++d) {
  draw_rationals(d, offset);
    offset -= 2;
}

// Force bottom margin to expand
draw(box((-1,1),(-0.5, -20)),white);
