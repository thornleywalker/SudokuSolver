
import tkinter as tk
import SudokuSolver
from numpy import *
from enum import Enum

class SquareType(Enum):
    ENTRY = 1
    DISPLAY = 2
class tkPossibility:
    def __init__(self, master):
        pass
class tkSquare:
    def __init__(self, master, type:SquareType):
        if type == ENTRY:
            pass
        elif type == DISPLAY:
            pass
class tkBox:
    def __init__(self, master, type:SquareType):
        pass
class tkBoard:
    def __init__(self, master, type:SquareType):
        pass

def main():
    print('Wassup fam')
    # Create window
    window = tk.Tk()
    # initial window has entry board for filling in
    # solve button - changes to possibility board
    frmBoard = tk.Frame(master = window,
                        relief=tk.RAISED,
                        borderwidth=5)
    frmBoard.grid(row=0, column=0)
    
    # real time display of solving
    # perhaps a speed setting/slider to slow solving down

    #run window mainloop
    window.mainloop()

if __name__ == '__main__': main()