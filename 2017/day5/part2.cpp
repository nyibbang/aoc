#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <array>
#include <string>
#include <cmath>
#include <sstream>

using namespace std;

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

auto read_input()
{
  vector<int> res;
  int v = 0;
  while (cin >> v)
    res.push_back(v);
  return res;
}

int main(int, char**)
{
  auto input = read_input();
  cout << "Input " << input << "\n";
  int it = 0;
  int step_count = 0;
  while (it >= 0 && it < input.size())
  {
    auto j = input[it];
    if (input[it] >= 3)
      --input[it];
    else
      ++input[it];
    it += j;
    //cout << "Jumped of " << j << " steps, now at " << it << ".\n";
    ++step_count;
  }

  cout << step_count << "\n";
}
