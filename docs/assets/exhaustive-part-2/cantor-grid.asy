unitsize(1cm);
int size = 5;
for (int x = 0; x < size; ++x) {
  for (int y = 0; y < size; ++y) {
    dot((x, y));
  }
}

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

path boundary = box((-0.5, -0.5), (size + 0.5, size + 0.5));
pair previous = (0, 0);
for (int i = 1; i < 15; ++i) {
  pair next = unpair(i);
  bool previous_in_range = previous.x < size && previous.y < size;
  bool next_in_range = next.x < size && next.y < size;
  if (previous_in_range && next_in_range) {
      draw(previous -- next, arrow=Arrow, gray);
  } else {
      draw(previous -- next, gray+Dotted());
  }
  previous = next;
}
clip(boundary);