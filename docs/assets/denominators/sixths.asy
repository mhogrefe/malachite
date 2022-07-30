import graph;
size(10cm);
guide gaxis=(0,0)--(20,0);
pen small=fontcommand("\footnotesize");
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

int gcd(int a, int b) {
  while (b != 0) {
    int t = b;
    b = a % b;
    a = t;
  }
  return a;
}

for (int i= 0; i <= 12; ++i) {
  if (gcd(i, 6) == 1) {
    dot((i*10/6, 0), red);
    label("$" + string(i) + "/6$",relpoint(gaxis,i/(2*6)),-2*plain.I*reldir(gaxis,0.5), red+small);
  }
}

draw((1.8, 0.2)--(1.8, -0.2),blue+linewidth(0.5mm));
draw((1.8, 0)--(8.2, 0),blue+linewidth(0.5mm));
draw((8.2, 0.2)--(8.2, -0.2),blue+linewidth(0.5mm));
