defaultpen(fontsize(14pt));
size(400,400);
real height = 10;
real tag_width = 35;
real small_width = 26;
real text_offset = 17.5;
real text_offset_2 = 49;
real text_offset_3 = 104;
real col_2 = 90;
string small_label = "{\tt Natural::Small}";
string large_label = "{\tt Natural::Large}";

filldraw(box((-5,5),(121,-85)),rgb(150.0/255,150.0/255,150.0/255));

filldraw(box((col_2,-height),(col_2+small_width,-2*height)),lightblue);
label("{\tt 0xa8dc8417 }", (text_offset_3,-3*height/2));
filldraw(box((col_2,-2*height),(col_2+small_width,-3*height)),lightblue);
label("{\tt 0x000009af }", (text_offset_3,-5*height/2));

filldraw(box((col_2,-4*height),(col_2+small_width,-5*height)),lightblue);
label("{\tt 0xd2d335fb }", (text_offset_3,-9*height/2));
filldraw(box((col_2,-5*height),(col_2+small_width,-6*height)),lightblue);
label("{\tt 0xce285713 }", (text_offset_3,-11*height/2));
filldraw(box((col_2,-6*height),(col_2+small_width,-7*height)),lightblue);
label("{\tt 0x005dd267 }", (text_offset_3,-13*height/2));

draw((tag_width+small_width,-13*height/2) -- (70,-13*height/2));
draw((70,-13*height/2) -- (70,-3*height/2));
draw((70,-3*height/2) -- (col_2,-3*height/2), arrow=Arrow);
draw((tag_width+small_width,-15*height/2) -- (80,-15*height/2));
draw((80,-15*height/2) -- (80,-9*height/2));
draw((80,-9*height/2) -- (col_2,-9*height/2), arrow=Arrow);

filldraw(box((0,0),(tag_width,-height)),mediumgray);
label(small_label, (text_offset,-height/2));
filldraw(box((tag_width,0),(tag_width+small_width,-height)),lightblue);
label("{\tt 0x00000002 }", (text_offset_2,-height/2));

filldraw(box((0,-height),(tag_width,-2*height)),mediumgray);
label(small_label, (text_offset,-3*height/2));
filldraw(box((tag_width,-height),(tag_width+small_width,-2*height)),lightblue);
label("{\tt 0x00000003 }", (text_offset_2,-3*height/2));

filldraw(box((0,-2*height),(tag_width,-3*height)),mediumgray);
label(small_label, (text_offset,-5*height/2));
filldraw(box((tag_width,-2*height),(tag_width+small_width,-3*height)),lightblue);
label("{\tt 0x00000007 }", (text_offset_2,-5*height/2));

filldraw(box((0,-3*height),(tag_width,-4*height)),mediumgray);
label(small_label, (text_offset,-7*height/2));
filldraw(box((tag_width,-3*height),(tag_width+small_width,-4*height)),lightblue);
label("{\tt 0x0000002b }", (text_offset_2,-7*height/2));

filldraw(box((0,-4*height),(tag_width,-5*height)),mediumgray);
label(small_label, (text_offset,-9*height/2));
filldraw(box((tag_width,-4*height),(tag_width+small_width,-5*height)),lightblue);
label("{\tt 0x0000070f }", (text_offset_2,-9*height/2));

filldraw(box((0,-5*height),(tag_width,-6*height)),mediumgray);
label(small_label, (text_offset,-11*height/2));
filldraw(box((tag_width,-5*height),(tag_width+small_width,-6*height)),lightblue);
label("{\tt 0x0031cbd3 }", (text_offset_2,-11*height/2));

filldraw(box((0,-6*height),(tag_width,-7*height)),mediumgray);
label(large_label, (text_offset,-13*height/2));
filldraw(box((tag_width,-6*height),(tag_width+small_width,-7*height)),mediumgray);
label("{\tt Vec }", (text_offset_2,-13*height/2));

filldraw(box((0,-7*height),(tag_width,-8*height)),mediumgray);
label(large_label, (text_offset,-15*height/2));
filldraw(box((tag_width,-7*height),(tag_width+small_width,-8*height)),mediumgray);
label("{\tt Vec }", (text_offset_2,-15*height/2));
