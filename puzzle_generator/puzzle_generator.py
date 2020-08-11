from selenium.webdriver import Firefox
from selenium.webdriver.firefox.options import Options
import io
import threading

class SudokuGenerator:
    #definitions for mapping
    one = "M8.954 30V3.545h-.267c-.738.41-6.706 4.655-7.71 5.311V5.883c.635-.41 6.767-4.758 7.977-5.476h2.789V30h-2.79z"
    two = "M.12 9.57C.16 4.462 4.057.791 9.41.791c5.209 0 9.187 3.568 9.187 8.224 0 3.076-1.415 5.475-6.275 10.664l-7.998 8.53v.247h15.012V31H.284v-1.969L10.62 17.854c4.122-4.43 5.168-6.193 5.168-8.736 0-3.302-2.81-5.865-6.44-5.865-3.814 0-6.46 2.584-6.5 6.316v.02H.12v-.02z"
    three = "M6.698 16.932v-2.42h3.466c3.814 0 6.46-2.338 6.46-5.722 0-3.22-2.646-5.537-6.317-5.537-3.67 0-6.255 2.174-6.542 5.537H1.038C1.366 3.95 5.037.792 10.41.792c5.045 0 9.064 3.404 9.064 7.67 0 3.568-2.05 6.173-5.496 6.932v.266c4.225.472 6.85 3.281 6.85 7.342 0 4.86-4.491 8.613-10.295 8.613-5.722 0-9.926-3.404-10.11-8.182H3.13c.246 3.322 3.322 5.721 7.382 5.721 4.286 0 7.424-2.645 7.424-6.214 0-3.711-2.912-6.008-7.65-6.008H6.699z"
    four = "M15.855 30v-6.686H.987v-2.563C3.633 16.281 7.283 10.6 14.563.366h4.02v20.426h4.43v2.522h-4.43V30h-2.728zM3.92 20.628v.184h11.935V3.052h-.184C10.03 10.744 7.099 15.338 3.92 20.629z"
    five = "M10.553 30.615c-5.373 0-9.474-3.445-9.782-8.264H3.52c.308 3.322 3.322 5.783 7.055 5.783 4.327 0 7.424-3.097 7.424-7.445 0-4.347-3.097-7.444-7.363-7.444-2.912 0-5.496 1.415-6.747 3.692H1.222l1.6-16.53h16.14V2.93H5.037l-.985 10.787h.267c1.415-1.846 3.876-2.912 6.768-2.912 5.68 0 9.72 4.08 9.72 9.802 0 5.866-4.245 10.008-10.254 10.008z"
    six = "M10.964 31.595c-4 0-7.158-1.99-9.003-5.64C.648 23.638-.01 20.582-.01 16.83-.008 6.76 4.135.792 11.17.792c4.901 0 8.613 2.953 9.454 7.567h-2.871c-.739-3.076-3.323-5.045-6.624-5.045-5.312 0-8.347 4.963-8.409 13.74h.246c1.292-3.322 4.553-5.454 8.43-5.454 5.618 0 9.76 4.183 9.76 9.843 0 5.886-4.285 10.152-10.191 10.152zm-.041-2.482c4.204 0 7.403-3.281 7.403-7.567 0-4.368-3.097-7.506-7.383-7.506-4.225 0-7.485 3.158-7.485 7.3 0 4.41 3.24 7.773 7.465 7.773z"
    seven = "M3.017 30L16.696 3.155V2.93H.29V.407h19.277v2.625L6.01 30z"
    eight = "M10.533 31.615c-6.193 0-10.48-3.527-10.48-8.593 0-3.834 2.584-6.87 6.46-7.567v-.246c-3.22-.759-5.311-3.343-5.311-6.583 0-4.573 3.876-7.834 9.33-7.834 5.456 0 9.332 3.24 9.332 7.834 0 3.22-2.071 5.804-5.291 6.583v.246c3.855.697 6.46 3.732 6.46 7.567 0 5.086-4.286 8.593-10.5 8.593zm0-2.42c4.532 0 7.67-2.604 7.67-6.357 0-3.671-3.117-6.173-7.67-6.173-4.532 0-7.65 2.523-7.65 6.173 0 3.753 3.118 6.357 7.65 6.357zm0-14.95c3.896 0 6.562-2.174 6.562-5.393 0-3.343-2.666-5.64-6.562-5.64-3.897 0-6.563 2.297-6.563 5.64 0 3.199 2.666 5.393 6.563 5.393z"
    nine = "M10.897 31.595c-4.983 0-8.613-2.974-9.454-7.547h2.871c.718 3.015 3.22 5.045 6.624 5.045 5.23 0 8.203-4.779 8.408-13.064.02-.205-.102-.471-.123-.676H19.1c-1.271 3.26-4.552 5.434-8.428 5.434-5.66 0-9.762-4.163-9.762-9.803C.91 5.1 5.175.792 11.102.792c4 0 7.157 2.01 9.003 5.68 1.313 2.298 1.969 5.333 1.969 9.106 0 10.028-4.102 16.017-11.177 16.017zm.226-13.248c4.245 0 7.485-3.2 7.485-7.28 0-4.39-3.22-7.794-7.465-7.794-4.224 0-7.403 3.302-7.403 7.63 0 4.285 3.035 7.444 7.383 7.444z"

    values = {
        one:'1',
        two:'2',
        three:'3',
        four:'4',
        five:'5',
        six:'6',
        seven:'7',
        eight:'8',
        nine:'9'
    }

    #headless browser setup
    opts = Options()
    opts.set_headless()
    assert opts.headless
    browser = Firefox(options=opts)

    def __init__(self):
        self.opts = Options()
        self.opts.set_headless()
        assert self.opts.headless
        self.browser = Firefox(options=self.opts)
        pass

    def generate(self, diff: str, num: int):
        self.browser.get('https://sudoku.com/{}/'.format(diff))
        #file output
        f = open('puzzles/{}.txt'.format(diff), 'a')
        
        for i in range(num):
            #parsing and creating game string
            game_row_list = self.browser.find_elements_by_class_name('game-row')
            game_string = ""
            for row in game_row_list:
                row_cell_values = row.find_elements_by_class_name('game-cell')
                for cell in row_cell_values:
                    try:
                        square_value = cell.find_element_by_tag_name('path')
                        game_string += SudokuGenerator.values[square_value.get_attribute('d')]
                        
                    except:
                        game_string += '0'
                        pass
                game_string += ','

            f.write(game_string)
            f.write('\n')
            self.browser.refresh()
            print('{} {} puzzles generated:'.format(i+1,diff))
        f.close()
        
    def end(self):
        self.browser.close()
        self.browser.quit()

def main():
    
    # for i in range(200):
    #     print("{}th iteration".format(i))
    #     num = 5
    #     SudokuGenerator.generate('easy', num)
    #     SudokuGenerator.generate('medium', num)
    #     SudokuGenerator.generate('hard', num)
    #     SudokuGenerator.generate('expert', num)
    easy_gen = SudokuGenerator()
    medium_gen = SudokuGenerator()
    hard_gen = SudokuGenerator()
    expert_gen = SudokuGenerator()

    easy_thread = threading.Thread(
        target=easy_gen.generate,
        args=('easy', 5),
        )

    medium_thread = threading.Thread(
        target=medium_gen.generate,
        args=('medium', 5),
        )

    hard_thread = threading.Thread(
        target=hard_gen.generate,
        args=('hard', 5),
        )
    expert_thread = threading.Thread(
        target=expert_gen.generate,
        args=('expert', 5),
        )

    easy_thread.start()
    medium_thread.start()
    hard_thread.start()
    expert_thread.start()

    for i in range(5):
        easy_thread.join()
        medium_thread.join()
        hard_thread.join()
        expert_thread.join()

        print("{}th iteration".format(i))

        print('running threads')
        easy_thread.start()
        medium_thread.start()
        hard_thread.start()
        expert_thread.start()

    easy_thread.join()
    medium_thread.join()
    hard_thread.join()
    expert_thread.join()
    
    print('threads complete')
    easy_gen.end()
    medium_gen.end()
    hard_gen.end()
    expert_gen.end()


if __name__ == '__main__': main()



