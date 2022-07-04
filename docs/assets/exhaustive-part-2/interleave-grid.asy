unitsize(1cm);
int size = 8;
for (int x = 0; x < size; ++x) {
  for (int y = 0; y < size; ++y) {
    dot((x, y));
  }
}

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

path boundary = box((-0.5, -0.5), (size + 0.5, size + 0.5));
pair previous = (0, 0);
for (int i = 1; i < size * size; ++i) {
  pair next = unpair(i);
  bool previous_in_range = previous.x <= size && previous.y <= size;
  bool next_in_range = next.x <= size && next.y <= size;
  if (previous_in_range && next_in_range) {
      draw(previous -- next, arrow=Arrow, gray);
  } else {
      draw(previous -- next, gray+Dotted());
  }
  previous = next;
}
clip(boundary);