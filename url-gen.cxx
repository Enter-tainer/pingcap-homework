#include <bits/stdc++.h>
using namespace std;

constexpr size_t G = 1000 * 1000 * 1000;

constexpr size_t file_size = 10 * G;
constexpr size_t repeat_time_mean = 200;
constexpr size_t repeat_time_stdddev = 1000;
constexpr double repeat_prob = 0.01;

string gen_random_string() {
  // [10, 2000)
  static const char alphanum[] = "0123456789"
                                 "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
                                 "abcdefghijklmnopqrstuvwxyz";
  static std::mt19937 rng(std::random_device{}());
  static std::uniform_int_distribution<> dist(0, sizeof(alphanum) - 2);
  static std::normal_distribution<> gen_len(300, 50);
  auto randchar = []() { return alphanum[dist(rng)]; };
  string tmp_s;
  tmp_s.resize(gen_len(rng));
  std::generate_n(tmp_s.begin(), tmp_s.length(), randchar);
  return tmp_s;
}

int main() {
  size_t byte_cnt = 0;
  ofstream f("urls.txt");
  static std::mt19937 rng(std::random_device{}());
  static std::uniform_real_distribution<> real_gen(0, 1);
  static std::normal_distribution<> repeat(repeat_time_mean,
                                           repeat_time_stdddev);

  size_t max_repeat_time = 0;
  string max_repeated;

  for (int i = 0; byte_cnt < file_size; ++i) {
    auto tmp = gen_random_string();
    if (real_gen(rng) <= repeat_prob) {
      double d_repeat_time = repeat(rng);
      if (d_repeat_time < 0) {
        continue;
      }
      size_t repeat_time = d_repeat_time;
      if (repeat_time > max_repeat_time) {
        max_repeat_time = repeat_time;
        max_repeated = tmp;
      }
      for (size_t j = 0; j < repeat_time && byte_cnt < file_size; ++j) {
        byte_cnt += tmp.size();
        f << tmp << "\n";
      }
    } else {
      byte_cnt += tmp.size();
      f << tmp << "\n";
    }
  }
  f.flush();
  cout << max_repeated << endl;
  cout << max_repeat_time << endl;
}
