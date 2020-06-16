# each row/column can only include one of each number
# each 9-square can only include one of each number

# concept: start with every number possible in every spot,
# eliminate numbers as they are made impossible
from numpy import *
import tkinter as tk

class Possibilities:
    def __init__(self):
        self.__internal = [1, 2, 3, 4, 5, 6, 7, 8, 9]
        pass
    def remove(self, number):
        self.__internal.remove(number)
        pass
    def getList(self):
        return self.__internal
    def contains(self, value):
        return self.__internal.count(value) > 0
    def emptyPossibilities(self):
        self.__internal.clear()
        pass

class OneSquare:
    def __init__(self):
        self.value = 0
        self.options = Possibilities()
        pass
    def setValue(self, val):
        self.value = val
        self.options.emptyPossibilities()
        pass

class ThreeSquare:
    def __init__(self):
        self.__internal = array([[OneSquare(),OneSquare(),OneSquare()],
                            [OneSquare(),OneSquare(),OneSquare()],
                            [OneSquare(),OneSquare(),OneSquare()]])
        pass
    def at(self, row, col):
        return self.__internal[row-1, col-1]


class Board:
    def __init__(self):
        self.__internal = array([[ThreeSquare(),ThreeSquare(),ThreeSquare()],
                            [ThreeSquare(),ThreeSquare(),ThreeSquare()],
                            [ThreeSquare(),ThreeSquare(),ThreeSquare()]])
        pass
    def at(self, row, col):
        threeRow = (row-1) // 3
        oneRow = (row-1) % 3
        threeCol = (col-1) // 3
        oneCol = (col-1) % 3

        return self.__internal[threeRow,threeCol].at(oneRow,oneCol)
    def checkSquare(self, row, col):
        # if something changes, return True
        # otherwise return False
        pass
    def toString(self):
        outString = ''
        for row in range(1,10):
            outString += '| '
            for col in range(1,10):
                outString += str(self.at(row, col).value)
                outString += ' '
            outString += '|\n\r'
        return outString

def main():

    displayWindow = tk.Tk()
    for i in range(1):
        displayWindow.columnconfigure(i, weight=1, minsize=50)
    
    displayWindow.rowconfigure(1, weight=1, minsize=50)

    # sudoku number squares
    fr_squares = tk.Frame(
        master=displayWindow,
        relief=tk.RAISED,
        borderwidth=2,
        padx=10,
        pady=10
    )
    fr_squares.grid(row=1,column=0)
    
    # interaction panel
    fr_panel = tk.Frame(
        master=displayWindow,
        relief=tk.FLAT
    )
    fr_panel.grid(row=1, column=1, padx=10)

    btn_solve = tk.Button(
        master=fr_panel,
        text='Solve',
    )
    btn_solve.grid(row=2, pady=5)

    btn_clear = tk.Button(
        master=fr_panel,
        text='Clear',
    )
    btn_clear.grid(row=1, pady=5)

    frames = {}
    entries = {}
    for i in range(1,10):
        for j in range(1,10):
            frames[str(i)+str(j)] = tk.Frame(
                master = fr_squares,
                relief = tk.FLAT,
                borderwidth=1,
                width=20,
                height=20
            )
            frames[str(i)+str(j)].grid(row=i, column=j)
            entries[str(i)+str(j)] = tk.Entry(
                master=frames[str(i)+str(j)],
                width=5
                )
            entries[str(i)+str(j)].insert(0, str(i)+str(j))
            entries[str(i)+str(j)].pack()
    
    displayWindow.mainloop()




    print('beginning sudoku solver')
    currBoard = Board()

    currBoard.at(2,2).setValue(5)
    currBoard.at(5,5).setValue(7)
    currBoard.at(7,7).setValue(9)

    print(currBoard.toString())

    # loop until nothing changes
    boardChanged = True
    while boardChanged:
        boardChanged = False
        for row in range(1, 10):
            for col in range(1, 10):
                #print('checking {},{} = {}'.format(row, col, currBoard.at(row, col).value))

                if currBoard.checkSquare(row, col): boardChanged = True
    pass

if __name__ == '__main__': main()