
import tkinter as tk
import tkinter.font as tkFont
import SudokuSolver as ss
from numpy import *
from enum import Enum

class SquareType(Enum):
    ENTRY = 1
    DISPLAY = 2
class tkPossibilities:
    def __init__(self, master, options:ss.Possibilities):
        possibilityFont = tkFont.Font(family = 'Ariel', size=7)
        self.__internal = tk.Frame(master=master,
                                   height=60,
                                   width=60)
        for row in range(3):
            for col in range(3):
                if options.getList().count(row*3+col+1)!=0:
                    label = tk.Label(master=self.__internal,
                                     text=row*3+col+1,
                                     font=possibilityFont,
                                     height=1,
                                     width=2)
                    label.grid(row=row, column=col)
                else:
                    label = tk.Label(master=self.__internal,
                                     text='',
                                     font=possibilityFont,
                                     height=1,
                                     width=2)
                    label.grid(row=row, column=col)
    def grid(self, row, col):
        self.__internal.grid(row=row, column=col)
class tkSquare:
    def __init__(self):
        self.__internal = None
    def init(self, master, type:SquareType, oneSquare:ss.OneSquare=None):
        self.numberFont = tkFont.Font(family='Ariel', size=30)
        self.__internal = tk.Frame(master=master,
                                   relief=tk.RAISED,
                                   borderwidth=2,
                                   height=60,
                                   width=60)
        if type == SquareType.ENTRY:
            self.entry = tk.Entry(master=self.__internal,width=3)
            self.entry.pack()
        elif type == SquareType.DISPLAY:
            if oneSquare==None:
                self.label = tk.Label(self.__internal,
                                        text='err',
                                        font=numberFont,
                                        height=1,
                                        width=2)
                self.label.pack()
            else:
                pass
        else:
            self.__internal = None
    def grid(self, row, column):
        if self.__internal != None:
            self.__internal.grid(row=row, column=column)
    def getInput(self):
        returnString = str(self.entry.get())
        if len(returnString) == 0:
            return '0'
        else: return returnString
    def displayView(self, oneSquare:ss.OneSquare):
        try:
            self.entry.pack_forget()
            self.label.pack_forget()
        except:
            pass
        if oneSquare.value == 0:
            #create possibilities grid
            self.__possibilities = tkPossibilities(self.__internal, oneSquare.options)
            self.__possibilities.grid(row=0, col=0)
        else:
            self.label = tk.Label(self.__internal,
                                  text=oneSquare.value,
                                  font=self.numberFont,
                                  height=1,
                                  width=2)
            self.label.grid(row=0, column=0)
class tkBox:
    def __init__(self):
        self.__internal = None
    def init(self, master, type:SquareType):
        self.__internal = tk.Frame(master=master,
                                   relief=tk.SUNKEN,
                                   width=70,
                                   height=70,
                                   borderwidth=5,
                                   )
        self.array = array([[tkSquare(),tkSquare(),tkSquare(),],
                            [tkSquare(),tkSquare(),tkSquare(),],
                            [tkSquare(),tkSquare(),tkSquare(),]])
        for row in range(3):
            for col in range(3):
                self.array[row, col].init(self.__internal, type,)
                self.array[row, col].grid(row=row, column=col)
    def grid(self, row, column):
        if self.__internal != None:
            self.__internal.grid(row=row, column=column)
class tkBoard:
    def __init__(self, master, type:SquareType):
        self.__board = ss.Board()
        #fillString ="""385000000,
                   #001009000,
                   #002061000,
                   #020050008,
                   #000030000,
                   #000100035,
                   #000704600,
                   #800000200,
                   #070000010"""
        #self.__board.fill(fillString)
        self.__internal = tk.Frame(master=master,
                                   relief=tk.RAISED,
                                   borderwidth=2,
                                   )
        self.__internal.grid(row=0, column=0, padx=10, pady=10)
        self.array = array([[tkBox(),tkBox(),tkBox(),],
                              [tkBox(),tkBox(),tkBox(),],
                              [tkBox(),tkBox(),tkBox(),]])
        for row in range(3):
            for col in range(3):
                self.array[row, col].init(self.__internal, type,
                                            )
                self.array[row, col].grid(row=row, column=col)
    def solve(self):
        # Get input currently in squares, convert to fill string
        # Solve
        self.__board.solve()
        self.displayValues()
    def getInput(self):
        fillString = ''
        for row in range(9):
            for col in range(9):
                fillString += self.array[row//3, col//3].array[row%3, col%3].getInput()
        print(fillString)
        self.__board.fill(fillString)
        self.displayValues()
    def displayValues(self):
        for row in range(9):
            for col in range(9):
                self.array[row//3, col//3].array[row%3, col%3].displayView(self.__board.at(row+1, col+1))

class tkWindow:
    def __init__(self):
        self.__internal = tk.Tk()
        # initial window has entry board for filling in
        self.__board = tkBoard(self.__internal, SquareType.ENTRY)

        # solve button - changes to display board
        self.__controlPanel = tk.Frame(master=self.__internal,
                                       )
        self.__controlPanel.grid(row=0,column=1, padx=10, pady=10)
        self.__solveButton = tk.Button(master=self.__controlPanel,
                                text='Submit',
                                width=10,
                                height=2,
                                command=self.getInput
                                )
        self.__solveButton.grid(row=2, column=0)
        # real time display of solving

        # perhaps a speed setting/slider to slow solving down
        
    def mainloop(self):
        self.__internal.mainloop()
    def getInput(self):
        self.__board.getInput()
        self.__solveButton = tk.Button(master=self.__controlPanel,
                                text='Solve',
                                width=10,
                                height=2,
                                command=self.solve
                                )
        self.__solveButton.grid(row=2, column=0)
    def solve(self):
        self.__board.solve()
        self.__solveButton = tk.Button(master=self.__controlPanel,
                                text='New Game',
                                width=10,
                                height=2,
                                command=self.newGame
                                )
        self.__solveButton.grid(row=2, column=0)
    def newGame(self):
        temp = tkWindow()
        self.__internal.destroy()
        self = temp
def main():
    print('Wassup fam')
    # Create window
    window = tkWindow()
   
    #run window mainloop
    window.mainloop()

if __name__ == '__main__': main()