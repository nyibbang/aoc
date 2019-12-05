#include <iostream>
#include <algorithm>
#include <vector>
#include <string>
#include <cmath>
#include <tuple>
#include <map>

using namespace std;

int read_input()
{
  int v = 0;
  cin >> v;
  return v;
}

template<typename T, typename T2>
ostream& operator<<(ostream& os, const pair<T, T2>& p)
{
  os << "(" << p.first << ", " << p.second << ")";
  return os;
}

template<typename T>
ostream& operator<<(ostream& os, const vector<T>& tv)
{
  bool first = true;
  os << "[";
  for (const auto& v : tv)
  {
    if (!first)
      os << ", ";
    first = false;
    os << v;
  }
  os << "]";
  return os;
}

template<typename T, typename V>
ostream& operator<<(ostream& os, const map<T, V>& tv)
{
  bool first = true;
  os << "[";
  for (const auto& v : tv)
  {
    if (!first)
      os << ", ";
    first = false;
    os << v;
  }
  os << "]";
  return os;
}

struct Coordinates
{
  int x;
  int y;

  bool operator<(Coordinates o) const
  {
    return make_tuple(x, y) < make_tuple(o.x, o.y);
  }

  bool operator==(Coordinates o) const
  {
    return make_tuple(x, y) == make_tuple(o.x, o.y);
  }

  Coordinates operator+(Coordinates o) const
  {
    return {x + o.x, y + o.y};
  }
};

ostream& operator<<(ostream& os, const Coordinates& c)
{
  os << "Coord{x=" << c.x << ", y=" << c.y << "}";
  return os;
}

enum class Direction
{
  Up,
  Down,
  Right,
  Left,
};

struct Movement
{
  Coordinates coord;
  Direction dir_to_next;
};

static map<Coordinates, int> visited;

Coordinates next(Coordinates coord, Direction dir)
{
  switch (dir)
  {
    case Direction::Up:    return coord + Coordinates{0, 1};
    case Direction::Down:  return coord + Coordinates{0, -1};
    case Direction::Right: return coord + Coordinates{1, 0};
    case Direction::Left:  return coord + Coordinates{-1, 0};
  }
  throw runtime_error("Fuck you");
}

Direction next(Direction dir, Coordinates coord)
{
  switch (dir)
  {
    case Direction::Up:
      if (visited.count(next(coord, Direction::Left)) == 0)
        return Direction::Left;
      else
        return Direction::Up;
    case Direction::Down:
      if (visited.count(next(coord, Direction::Right)) == 0)
        Direction::Right;
      else
        return Direction::Down;
    case Direction::Right:
      if (visited.count(next(coord, Direction::Up)) == 0)
        return Direction::Up;
      else
        return Direction::Right;
    case Direction::Left:
      if (visited.count(next(coord, Direction::Down)) == 0)
        return Direction::Down;
      else
        return Direction::Left;
  }
  throw runtime_error("Fuck you more");
}

Movement next(Movement mov)
{
  const auto new_coord = next(mov.coord, mov.dir_to_next);
  return Movement{new_coord, next(mov.dir_to_next, new_coord)};
}

vector<pair<Coordinates, int>> visited_neighbours(Coordinates coord)
{
  vector<Coordinates> neighbours {
    coord + Coordinates{1, 0},
    coord + Coordinates{1, 1},
    coord + Coordinates{0, 1},
    coord + Coordinates{-1, 1},
    coord + Coordinates{-1, 0},
    coord + Coordinates{-1, -1},
    coord + Coordinates{0, -1},
    coord + Coordinates{1, -1},
  };
  sort(begin(neighbours), end(neighbours));

  struct CompareCoordinates
  {
    bool operator()(const pair<Coordinates, int>& a, const Coordinates& b)
    {
      return a.first < b;
    }
    bool operator()(const Coordinates& a, const pair<Coordinates, int>& b)
    {
      return a < b.first;
    }
  };
  vector<pair<Coordinates, int>> res;
  set_intersection(begin(visited), end(visited),
                   begin(neighbours), end(neighbours),
                   back_inserter(res), CompareCoordinates{});
  return res;
}

int main(int, char**)
{
  const auto input = read_input();

  visited[{0, 0}] = 1;
  Movement mov{{0, 0}, Direction::Right};
  while (visited[mov.coord] <= input)
  {
    mov = next(mov);
    const auto vnb = visited_neighbours(mov.coord);
    const auto val = accumulate(begin(vnb), end(vnb),
        0, [](int v, const pair<Coordinates, int>& c) {
          return v + c.second;
        });
    visited[mov.coord] = val;
  }
  cout << "The first value written that is larger that the input is " << visited[mov.coord] << " at " << mov.coord << "\n";
}
