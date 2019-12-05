#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <cmath>

using namespace std;

int read_input()
{
  int v = 0;
  cin >> v;
  return v;
}

int square_arity(int index)
{
  if (index < 0)
    return 0;
  return 4 * index * index + 4 * index + 1;
}

int square_width(int index)
{
  if (index < 0)
    return 1;
  return 1 + index * 2;
}

int main(int, char**)
{
  const auto input = read_input();

  int index = 0;
  int arity = 0;
  while ((arity = square_arity(index)) < input)
  {
    cout << "Arity " << arity << " is lower or equal than " << input << "\n";
    ++index;
  }
  cout << "Index of square is=" << index << "\n";

  int x = 0;
  int y = 0;
  const auto width = square_width(index); // width of the side of the square
  if (width == 1)
  {
    x = 0;
    y = 0;
  }
  else
  {
    const auto arity_lower_bound = square_arity(index - 1);
    const auto index_in_square = input - arity_lower_bound - 1;
    const auto side_index = width > 1 ? index_in_square / (width - 1) : 0; // between 0 and 4
    const auto index_in_side = index_in_square % (width - 1); // between 0 and width - 1

    cout << "Arity lower bound=" << arity_lower_bound << "\n";
    cout << "Width of square=" << width << "\n";
    cout << "Index in square=" << index_in_square << "\n";
    cout << "Side index=" << side_index << "\n";
    cout << "Index in side=" << index_in_side << "\n";

    switch (side_index)
    {
      case 0:
        x = index;
        y = -(width/2 - 1) + index_in_side;
        break;
      case 1:
        y = index;
        x = width/2 - 1 - index_in_side;
        break;
      case 2:
        x = -index;
        y = width/2 - 1 - index_in_side;
        break;
      case 3:
        y = -index;
        x = -(width/2 - 1) + index_in_side;
        break;
    }
  }
  cout << "Square should be in x=" << x << ", y=" << y << "\n";
  cout << "Manhattan distance is " << abs(x) + abs(y) << "\n";
}
