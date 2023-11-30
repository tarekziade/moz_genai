#include <iostream>
#include <fstream>
#include "../target/summary.h" // Include the generated header file


int main() {
    char* result = ping();
    std::cout << result << std::endl;
    free_memory(result);


    std::cout << "Text version..." << std::endl;
    std::ifstream file("sandman.txt");

    if (file.is_open()) {
        std::string file_content((std::istreambuf_iterator<char>(file)),
                                 std::istreambuf_iterator<char>());

     const char* input = file_content.c_str();

      char* result = summarize_text(input);

      if (result != nullptr) {
          std::cout << "Result from Rust function: " << result << std::endl;

          free_memory(result);
      } else {
          std::cout << "Error processing the string in Rust." << std::endl;
      }
    } else {
      std::cout << "Unable to open file." << std::endl;
    }

    std::cout << "HTML version..." << std::endl;
    std::ifstream file2("sandman.html");

    if (file2.is_open()) {
        std::string file_content((std::istreambuf_iterator<char>(file2)),
                                 std::istreambuf_iterator<char>());

     const char* input = file_content.c_str();

      char* result = summarize_html(input);

      if (result != nullptr) {
          std::cout << "Result from Rust function: " << result << std::endl;

          free_memory(result);
      } else {
          std::cout << "Error processing the string in Rust." << std::endl;
      }
    } else {
      std::cout << "Unable to open file." << std::endl;
    }

    return 0;
}

