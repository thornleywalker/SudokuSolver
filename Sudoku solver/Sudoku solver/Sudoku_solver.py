# each row/column can only include one of each number
# each 9-square can only include one of each number

# concept: start with every number possible in every spot,
# eliminate numbers as they are made impossible
from numpy import *
import tkinter as tk

class Possibilities:
    def __init__(self):
        ###
        # Initializes internal list with all possibilities
        ###
        self.__internal = [1, 2, 3, 4, 5, 6, 7, 8, 9]
        pass
    def remove(self, number):
        ###
        # Removes given value from internal list
        ###
        try:
            self.__internal.remove(number)
        except:
            pass
        pass
    def getList(self):
        ###
        # Returns internal list
        ###
        return self.__internal
    def contains(self, value):
        ###
        # Returns True if given value is in internal list
        ###
        return self.__internal.count(value) > 0
    def emptyPossibilities(self):
        ###
        # Empties internal list
        ###
        self.__internal.clear()
        pass

class OneSquare:
    def __init__(self):
        ###
        # Initializes possibilities list and sets value to 0
        ###
        self.value = 0
        self.options = Possibilities()
        pass
    def setValue(self, val):
        ###
        # Sets value to given value
        # Empties possibilities
        ###
        self.value = val
        self.options.emptyPossibilities()
        pass
    def removePossibility(self, val):
        ###
        # Removes given value from possibilities
        ###
        self.options.remove(val)

class ThreeSquare:
    def __init__(self):
        ###
        # Initializes 3x3 array of OneSquares
        ###
        self.__internal = array([[OneSquare(),OneSquare(),OneSquare()],
                            [OneSquare(),OneSquare(),OneSquare()],
                            [OneSquare(),OneSquare(),OneSquare()]])
        pass
    def at(self, row, col):
        ###
        # Returns the OneSquare at the given location
        ###
        return self.__internal[row-1, col-1]
    def set(self, row, col, val):
        ###
        # Sets given square to given value.
        # Removes given value from possibilities of
        # other squares in the ThreeSquare
        ###
        self.__internal[row-1, col-1].setValue(val)
        for line in self.__internal:
            for square in line:
                square.removePossibility(val)

class Board:
    def __init__(self):
        ###
        # Initializes 3x3 array of ThreeSquares
        ##
        self.__internal = array([[ThreeSquare(),ThreeSquare(),ThreeSquare()],
                            [ThreeSquare(),ThreeSquare(),ThreeSquare()],
                            [ThreeSquare(),ThreeSquare(),ThreeSquare()]])
        pass
    def fill(self, inString:str):
        ###
        # Fills sudoku board with the given string's
        # values
        # format: 
        # inString = """123456789,
        #               123456789,
        #               ...
        #               123456789"""
        ###
        
        inString = inString.replace(",","").replace("\n","").replace(" ","")
        for i in range(len(inString)):
            col = (i % 9) + 1
            row = (i // 9) + 1
            val = int(inString[i])
            if(val!=0):
                self.set(row, col, val)
            pass
        pass
    def at(self, row, col):
        ###
        # Returns the OneSquare at the given location
        ###
        threeRow = (row-1) // 3
        oneRow = (row-1) % 3
        threeCol = (col-1) // 3
        oneCol = (col-1) % 3

        return self.__internal[threeRow,threeCol].at(oneRow,oneCol)
    def set(self, row, col, val):
        ###
        # Sets the given square to the given value.
        # Removes given value from possibilities of other
        # squares in the row, and the column.
        # ThreeSquare's set method removes given value from 
        # possibilities of other squares in the ThreeSquare
        ###
        threeRow = (row-1) // 3
        oneRow = (row-1) % 3
        threeCol = (col-1) // 3
        oneCol = (col-1) % 3

        self.__internal[threeRow,threeCol].set(oneRow,oneCol, val)

        #remove from all rows
        for i in range(1,10):
            self.at(i, col).removePossibility(val)
        #remove from all cols
        for i in range(1,10):
            self.at(row, i).removePossibility(val)
        pass
    def checkSquare(self, row, col):
        ###
        # Checks the given square for the following:
        # - it only has one possibility
        # - for each possibility in the square:
        #   - it is the only one of that possibility in the row
        #   - it is the only one of that possibility in the column
        # if any of these are true, it changes the square to the
        # corresponding possibility.
        # if something changes, returns True
        # otherwise returns False
        ###
        if(len(self.at(row, col).options.getList())==1):
            newVal = self.at(row, col).options.getList()[0]
            self.set(row, col, newVal)
            print('{},{} set to {}'.format(row, col, newVal))
            return True
        else:
            for checkVal in self.at(row, col).options.getList():
                #check ThreeSquare
                unique = True
                rStart = 3*((row-1)//3)+1
                rEnd = rStart + 3
                cStart = 3*((col-1)//3)+1
                cEnd = cStart + 3
                for checkRow in range(rStart, rEnd):
                    for checkCol in range(cStart,cEnd):
                        if(checkRow != row or checkCol != col):
                            if(self.at(checkRow, checkCol).options.contains(checkVal)):
                                unique = False
                if(unique):
                    self.set(row, col, checkVal)
                    print('{},{} set to {}'.format(row, col, checkVal))
                    return True

                #check the column
                unique = True
                for checkRow in range(1, 10):
                    if(checkRow != row):
                        if(self.at(checkRow, col).options.contains(checkVal)):
                            unique = False
                if(unique):
                    self.set(row, col, checkVal)
                    print('{},{} set to {}'.format(row, col, checkVal))
                    return True

                #check the row
                unique = True
                for checkCol in range(1, 10):
                    if(checkCol != col):
                        if(self.at(row, checkCol).options.contains(checkVal)):
                            unique = False
                if(unique):
                    self.set(row, col, checkVal)
                    print('{},{} set to {}'.format(row, col, checkVal))
                    return True
            return False
    def deepCheck(self):
        ###
        # Performs deep check:
        # - if a possibility can only be in one row/column of a ThreeSquare,
        #   removes that posibility from that row/column in adjacent ThreeSquares
        ###
        pass
    def solveCheck(self):
        ###
        # Checks if board is solved
        ###
        isSolved = True
        for row in range(1,10):
            for col in range(1,10):
                if(len(self.at(row, col).options.getList()) != 0):
                    isSolved = False
    def toString(self):
        ###
        # Returns ascii sudoku board
        ###
        outString = ''
        for row in range(1,10):
            outString += '| '
            for col in range(1,10):
                outString += str(self.at(row, col).value)
                outString += ' '
            outString += '|\n\r'
        return outString.replace('0', '_')

def main():

    print('beginning sudoku solver')
    currBoard = Board()

    ## testString
    fillString ="""230700001,
                   000300900,
                   060200400,
                   000000000,
                   020004100,
                   050600070,
                   005006009,
                   903000000,
                   010008500"""
    currBoard.fill(fillString)


    for i in range(1,10):
        for j in range(1,10):
            print('{},{}={}'.format(i,j,currBoard.at(i, j).options.getList()))
    print(currBoard.toString())

    # loop until solved
    boardChanged = True
    while boardChanged:
        boardChanged = False
        #basic solving
        for row in range(1, 10):
            for col in range(1, 10):
                if currBoard.checkSquare(row, col): boardChanged = True
        #deep check
        currBoard.deepCheck()
        pass

    for i in range(1,10):
        for j in range(1,10):
            print('{},{}={}'.format(i,j,currBoard.at(i, j).options.getList()))
    print(currBoard.toString())
    pass

if __name__ == '__main__': main()