#include <iostream>
#include <fstream>
#include <string>

int main(int argc, char* argv[]) {
    // Default input file path
    std::string inputFile = "input.txt";
    
    // Allow custom input file via command line argument
    if (argc > 1) {
        inputFile = argv[1];
    }
    
    std::ifstream file(inputFile);
    if (!file.is_open()) {
        std::cerr << "Error: Could not open file " << inputFile << std::endl;
        return 1;
    }
    
    const int DIAL_SIZE = 100;  // Numbers 0-99
    const int START_POSITION = 50;
    
    int position = START_POSITION;
    int zeroCountPart1 = 0;
    int zeroCountPart2 = 0;
    
    std::string line;
    while (std::getline(file, line)) {
        if (line.empty()) {
            continue;
        }
        
        // Parse the rotation instruction
        char direction = line[0];
        int distance = std::stoi(line.substr(1));
        
        // Part 2: Count how many times we pass through or land on 0
        // Calculate how many times we cross 0 during this rotation
        if (direction == 'L') {
            // Moving left (toward lower numbers)
            // We cross 0 each time we go from 0 to 99
            // Number of times = how many complete "wraps" plus if we pass through 0
            int newPos = position - distance;
            // Count crossings: we cross 0 when going from positive to negative (mod 100)
            // If position >= newPos: floor((position) / 100) - floor((newPos) / 100) when adjusted
            // Simpler: count how many times we'd hit 0
            // From position, going left by distance clicks
            // We hit 0 if we pass through it. Going left from pos, we hit 0, then 99, then 0 again...
            // Times hitting 0 = (position + 100 - (newPos % 100 + 100) % 100) / 100 if crossing
            
            // Count: how many multiples of 100 do we span?
            // Going from position down by distance
            // We hit 0 at positions: position, position-100, position-200, etc.
            // So we hit 0 when: position - k*100 where k >= 0 and position - k*100 >= newPos
            // That means: position >= k*100 + newPos, so k <= (position - newPos) / 100 = distance / 100
            // But we only count if we actually land on or pass 0
            // If position > 0, the first 0 we hit is at click number = position
            // Then every 100 clicks after that
            
            // Number of zeros hit = floor((position + distance) / 100) if position > 0
            // Or: (position + distance) / 100 rounded down, but only if distance > 0
            // Actually: starting at position, going left, we hit 0 after (position) steps (if position > 0)
            // Then every 100 steps after. If position == 0, we hit 0 after 100 steps.
            
            // Positions we pass through going left: position-1, position-2, ..., position-distance (mod 100)
            // We want to count how many of those equal 0.
            // This is equivalent to: how many k in [1, distance] have (position - k) % 100 == 0?
            // That means position - k ≡ 0 (mod 100), so k ≡ position (mod 100)
            // k must be in range [1, distance] and k ≡ position (mod 100)
            // First such k is: position (if position >= 1), or position + 100 if position == 0
            // Then position + 100, position + 200, etc.
            
            int firstK = (position == 0) ? DIAL_SIZE : position;
            if (firstK <= distance) {
                zeroCountPart2 += 1 + (distance - firstK) / DIAL_SIZE;
            }
            
            position = ((position - distance) % DIAL_SIZE + DIAL_SIZE) % DIAL_SIZE;
        } else if (direction == 'R') {
            // Moving right (toward higher numbers)
            // We hit 0 when position + k ≡ 0 (mod 100), so k ≡ -position ≡ 100 - position (mod 100)
            // For k in [1, distance]
            
            int firstK = (position == 0) ? DIAL_SIZE : (DIAL_SIZE - position);
            if (firstK <= distance) {
                zeroCountPart2 += 1 + (distance - firstK) / DIAL_SIZE;
            }
            
            position = (position + distance) % DIAL_SIZE;
        }
        
        // Part 1: Check if dial points at 0 after this rotation
        if (position == 0) {
            zeroCountPart1++;
        }
    }
    
    file.close();
    
    std::cout << "Part 1 - The actual password is: " << zeroCountPart1 << std::endl;
    std::cout << "Part 2 - The actual password is: " << zeroCountPart2 << std::endl;
    
    return 0;
}
