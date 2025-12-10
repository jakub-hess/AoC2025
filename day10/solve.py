import numpy as np
from scipy.optimize import milp, LinearConstraint, Bounds
import re
import time

def parse_line(line):
    """Parse a machine line into buttons and required joltages"""
    # Extract required joltages from {a,b,c,...}
    joltage_match = re.search(r'\{([^}]+)\}', line)
    required_joltages = list(map(int, joltage_match.group(1).split(',')))
    
    # Extract buttons from (a,b,c) patterns
    buttons = []
    for match in re.finditer(r'\(([^)]+)\)', line):
        indices = list(map(int, match.group(1).split(',')))
        buttons.append(indices)
    
    return buttons, required_joltages

def build_matrix(buttons, num_joltages):
    """Build coefficient matrix A where A[i][j] = 1 if button j affects joltage i"""
    num_buttons = len(buttons)
    A = np.zeros((num_joltages, num_buttons), dtype=np.int64)
    
    for j, button in enumerate(buttons):
        for idx in button:
            if idx < num_joltages:
                A[idx][j] = 1
    
    return A

def solve_machine(buttons, required_joltages):
    """
    Solve for minimum sum of non-negative integers x such that A @ x = b
    where A is the button matrix and b is required_joltages
    """
    num_joltages = len(required_joltages)
    A = build_matrix(buttons, num_joltages)
    b = np.array(required_joltages, dtype=np.int64)
    num_vars = len(buttons)
    
    # Use scipy's MILP solver
    # Minimize: sum(x)
    # Subject to: A @ x = b, x >= 0, x integer
    
    c = np.ones(num_vars)  # Minimize sum of all variables
    
    # Equality constraints: A @ x = b
    constraints = LinearConstraint(A, b, b)
    
    # Bounds: x >= 0 (and we'll treat as integers)
    bounds = Bounds(lb=0, ub=np.inf)
    
    # Integer constraints for all variables
    integrality = np.ones(num_vars)  # 1 = integer constraint
    
    result = milp(c, constraints=constraints, bounds=bounds, integrality=integrality)
    
    if result.success:
        solution = np.round(result.x).astype(int)
        # Verify solution
        computed = A @ solution
        if np.array_equal(computed, b) and all(x >= 0 for x in solution):
            return solution, int(sum(solution))
    
    return None, None

def main():
    start_time = time.time()
    total_sum = 0
    unsolved = 0
    
    with open('inputs-10.txt', 'r') as f:
        for idx, line in enumerate(f):
            line = line.strip()
            if not line:
                continue
                
            buttons, required_joltages = parse_line(line)
            solution, min_sum = solve_machine(buttons, required_joltages)
            
            if solution is not None:
                total_sum += min_sum
                # print(f"Machine {idx}: Sum = {min_sum}, Solution = {solution}")
            else:
                print(f"Machine {idx}: No solution found!")
                unsolved += 1
    
    elapsed = time.time() - start_time
    print(f"\nUnsolved: {unsolved}")
    print(f"Result P1: {total_sum}")
    print(f"Time: {elapsed*1000:.3f}ms")

if __name__ == "__main__":
    main()
