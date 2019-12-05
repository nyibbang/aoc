#include <iostream>
#include <algorithm>
#include <vector>
#include <set>
#include <map>
#include <array>
#include <string>
#include <cmath>
#include <cstring>
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

template<typename T, size_t N>
ostream& operator<<(ostream& os, const array<T, N>& tv)
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

template<typename T>
ostream& operator<<(ostream& os, const set<T>& tv)
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

struct Node
{
  string name;
  int weight = 0;

  bool operator<(const Node& o) const
  {
    return make_tuple(name, weight) < make_tuple(o.name, o.weight);
  }

  bool operator<(const string& s) const
  {
    return name < s;
  }
};

ostream& operator<<(ostream& os, const Node& n)
{
  os << "{name:" << n.name << ", weight:" << n.weight << "}";
  return os;
}

struct Tree
{
  set<Node> nodes;
  map<Node, Node> parents;
  multimap<Node, Node> children;
};

auto read_input()
{
  Tree tree;
  string line;
  multimap<Node, string> intermediate;
  while (getline(cin, line))
  {
    Node node;
    string children;
    {
      char raw_name[16];
      fill(begin(raw_name), end(raw_name), 0);
      char raw_children[128];
      fill(begin(raw_children), end(raw_children), 0);
      sscanf(line.data(), "%s (%d) -> %[a-zA-Z, ]", raw_name, &node.weight,
             raw_children);
      node.name = string{raw_name, strlen(raw_name)};
      children = string{raw_children, strlen(raw_children)};
    }
    cout << "Node read: " << node << ", children=" << children << "\n";
    tree.nodes.insert(node);

    istringstream ciss{children};
    string child;
    while (getline(ciss, child, ','))
    {
      intermediate.insert({node, child});
      ciss.get();  // ignore space
    }
  }

  for (const auto& intermvt : intermediate)
  {
    const auto& node = intermvt.first;
    const auto& childname = intermvt.second;
    const auto& childnode =
        *find_if(begin(tree.nodes), end(tree.nodes), [&](const auto& vt) {
          return childname == vt.name;
        });
    cout << "Child of " << node << ": " << childnode << "\n";
    tree.parents[childnode] = node;
    tree.children.insert({node, childnode});
  }

  return tree;
}

int total_weight(const Node& n, const Tree& t)
{
  const auto children_range = t.children.equal_range(n);
  auto res = accumulate(children_range.first, children_range.second, n.weight,
                    [&](auto curr, const auto& elem) {
                      return curr + total_weight(elem.second, t);
                    });
  //cout << "Node " << n << " has a total weight of " << res << "\n";
  return res;
}

bool is_balanced(const Node& n, const Tree& t)
{
  const auto children_range = t.children.equal_range(n);
  return adjacent_find(
      children_range.first, children_range.second,
      [&](const auto& vt1, const auto& vt2) {
        return total_weight(vt1.second, t) != total_weight(vt2.second, t);
      }) == children_range.second;
}

int main(int, char**)
{
  const auto tree = read_input();

  const auto unbalanced_leaf = [&](const auto& node) {
    const auto balanced = is_balanced(node, tree);
    if (balanced) return false;
    const auto children_range = tree.children.equal_range(node);
    return all_of(
        children_range.first, children_range.second,
        [&](const auto& elem) { return is_balanced(elem.second, tree); });
  };

  auto c = count_if(begin(tree.nodes), end(tree.nodes), unbalanced_leaf);
  cout << "There are " << c << " leaves than are unbalanced.\n";

  auto it = find_if(begin(tree.nodes), end(tree.nodes), unbalanced_leaf);
  if (it != end(tree.nodes))
  {
    const auto& node = *it;
    cout << "Node " << node << " is unbalanced.\n";
    const auto children_range = tree.children.equal_range(node);
    cout << "Node " << node << " children are:\n";
    for (auto it = children_range.first; it != children_range.second; ++it)
    {
      cout << "\t- Node " << it->second << " which has a total weight of " << total_weight(it->second, tree) << "\n";
    }
  }
  else
  {
    cout << "No unbalanced node was found\n";
  }
}
