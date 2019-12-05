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
  vector<vector<string>> res;
  string line;
  while (getline(cin, line))
  {
    vector<string> words;
    istringstream iss(line);
    string word;
    while (getline(iss, word, ' '))
      words.push_back(move(word));
    res.push_back(move(words));
  }
  return res;
}

bool has_duplicates(const vector<string>& words)
{
  auto wcpy = words;
  sort(begin(wcpy), end(wcpy));
  return adjacent_find(begin(wcpy), end(wcpy)) != end(wcpy);
}

int main(int, char**)
{
  const auto input = read_input();
  cout << accumulate(begin(input), end(input), 0,
      [](int res, const auto& w){ const auto d = has_duplicates(w);
        return d ? res : res + 1;
      }) << "\n";
}
