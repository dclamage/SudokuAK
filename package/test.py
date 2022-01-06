import time
import sudoku_ak

start_time = time.time()
count = sudoku_ak.solution_count('..................................1.......2.34....5....6...7..........8..........', 0)
end_time = time.time()
print('Solution count: ' + str(count) + (count == 3 and ' (correct)' or ' (incorrect)'))
elapsed_time_sec = end_time - start_time
elapsed_time_ms = elapsed_time_sec * 1000.0
print("--- %0.3fms ---" % elapsed_time_ms)

solved = sudoku_ak.solve('..................................1.......2.34....5....6...7..........8..........', False)
solved_random = sudoku_ak.solve('..................................1.......2.34....5....6...7..........8..........', True)
print('Solved: ' + str(solved))
print('Solved (Random): ' + str(solved_random))

minlexed = sudoku_ak.minlex('9.......1......7....8......7.....2...4..68..........5..6..7..............3.....7.')
print('Minlexed: ' + minlexed + (minlexed == '........12..3..........4.2.....5......2.6...................5..7.6.8.........2..9' and ' (correct)' or ' (incorrect)'))
