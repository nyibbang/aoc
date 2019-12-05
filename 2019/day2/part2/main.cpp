#include <iostream>
#include <iterator>
#include <vector>
#include <fstream>
#include <cassert>

template<typename Fn>
std::size_t opcode_3pos(std::vector<int>& codes, std::size_t address, Fn op) {
  auto instr = codes.at(address);
  auto in_pos1 = codes.at(++address);
  auto in_pos2 = codes.at(++address);
  auto out_pos = codes.at(++address);
  //std::cerr << "op: " << instr << "," << in_pos1 << "," << in_pos2 << "," << out_pos << std::endl;
  codes.at(out_pos) = op(codes.at(in_pos1), codes.at(in_pos2));
  return ++address;
}

std::size_t opcode_1(std::vector<int>& codes, std::size_t address) {
  assert(codes.at(address) == 1);
  return opcode_3pos(codes, address, std::plus<int>{});
}

std::size_t opcode_2(std::vector<int>& codes, std::size_t address) {
  assert(codes.at(address) == 2);
  return opcode_3pos(codes, address, std::multiplies<int>{});
}

void print_out(const std::vector<int>& codes) {
  std::cout << codes.at(1) << ", " << codes.at(2)
            << " => " << 100 * codes.at(1) + codes.at(2)
            << std::endl;
}

int main() {
  std::ifstream in("input.txt");

  std::vector<int> codes;
  while(in && !in.eof()) {
    int code = 0;
    in >> code;
    codes.push_back(code);
    if (in.eof())
      break;
    in.ignore(std::numeric_limits<std::streamsize>::max(), ',');
  }
  auto codes_orig = codes;

  assert(codes.size() > 3);

  for (int noun = 0; noun < 99; ++noun) {
    for (int verb = 0; verb < 99; ++verb) {
      codes.at(1) = noun;
      codes.at(2) = verb;

      std::size_t address = 0;
      bool halt = false;
      while (!halt) {
        assert(address < codes.size());
        switch(codes.at(address)) {
          case 1:
            address = opcode_1(codes, address);
            break;
          case 2:
            address = opcode_2(codes, address);
            break;
          case 99:
            //std::cerr << "halt" << std::endl;
            halt = true;
            break;
        }
      }

      //std::cerr << "noun=" << noun << ", verb=" << verb << ", out=" << codes.at(0) << std::endl;
      if (codes.at(0) == 19690720) {
        print_out(codes);
        return 0;
      }

      codes = codes_orig;
    }
  }

  std::cerr << "no input found for that output" << std::endl;
  std::terminate();
}
