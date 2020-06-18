
import tkinter as tk
import SudokuSolver as ss
from numpy import *
from enum import Enum

class SquareType(Enum):
    ENTRY = 1
    DISPLAY = 2
class tkPossibility:
    def __init__(self, master):
        pass
class tkSquare:
    def __init__(self):
        self.__internal = None
    def init(self, master, type:SquareType):
        if type == SquareType.ENTRY:
            self.__internal = tk.Frame(master=master,
                                       relief=tk.RAISED,
                                       borderwidth=2,
                                       height=60,
                                       width=60)
            self.entry = tk.Entry(master=self.__internal,width=3)
            self.entry.pack(fill='both')
            self.__internal.config(width=60, height=60)
        elif type == SquareType.DISPLAY:
            self.__internal = tk.Frame(master=master,
                                        relief=tk.RAISED,
                                        borderwidth=2,
                                        height=60,
                                        width=60)
            self.label = tk.Label(self.__internal,
                                    text=5,
                                    font='ariel',
                                    height=3,
                                    width=6)
            self.label.pack()
        else:
            self.__internal = None
    def grid(self, row, column):
        if self.__internal != None:
            self.__internal.grid(row=row, column=column)
    def getInput(self):
        pass
class tkBox:
    def __init__(self):
        self.__internal = None
    def init(self, master, type:SquareType):
        self.__internal = tk.Frame(master=master,
                                   relief=tk.SUNKEN,
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
    def getInput(self):
        pass
class tkBoard:
    def __init__(self, master, type:SquareType):
        self.__board = ss.Board()
        fillString ="""385000000,
                   001009000,
                   002061000,
                   020050008,
                   000030000,
                   000100035,
                   000704600,
                   800000200,
                   070000010"""
        self.__board.fill(fillString)
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
        self.__board.solve()
    def getInput(self):
        fillString = str
        for row in range(3):
            for col in range(3):
                fillString += self.array[row, col].getInput()
        self.__board.fill(fillString)
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
                                text='Solve',
                                width=10,
                                height=2
                                )
        self.__solveButton.grid(row=2, column=0)
        # real time display of solving

        # perhaps a speed setting/slider to slow solving down
        
    def mainloop(self):
        self.__internal.mainloop()
def main():
    print('Wassup fam')
    # Create window
    window = tkWindow()
   
    #run window mainloop
    window.mainloop()

if __name__ == '__main__': main()