
from excel import ExcelHandle

book = ExcelHandle('./python_sample.xlsx')


# find_sheets
func = lambda x: x == "Hoge"
sheet_list = book.find_sheets(range(2, 10), [1], func)
print(sheet_list)

func = lambda x: False if type(x) != type(0) else x > 5
sheet_list = book.find_sheets([3], range(2, 5), func)
print(sheet_list)


# find_cell
func = lambda x: x == "Foo"
address = book.find_cell('Sheet2', range(3, 10), range(3, 10), func)
print(address, '\n')

address = book.find_cell('Sheet2', reversed(range(3, 10)), range(3, 10), func)
print(address, '\n') 

func = lambda x: x is None
_, col = book.find_cell('Sheet3', range(6, 9), range(2, 8, 2), func) 
print(col, '\n')


# iterate_row_values
for values in book.iterate_row_values('Sheet1', [9, 10], range(2, 5, 2), lambda x:False): 
    print(*values, "\n")

def cols(n):
    while True:
        yield n
        n += 1
func = lambda xs: False in [x is not None for x in xs] 
for values in book.iterate_row_values('Sheet1', [1, 5, 6], cols(2), func): 
    print(*values, "\n") 

for v1, v2 in zip( 
            book.iterate_row_values('Sheet1', [6], cols(3), func),
            book.iterate_row_values('Sheet3', [6], cols(3), func)
        ):
    print(v1[0] + v2[0]) 



    