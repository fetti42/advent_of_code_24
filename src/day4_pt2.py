import argparse


def import_data(filename):
    data = [i.strip('\n') for i in open(filename).readlines()]
    return(data)



def run_test(data):
    a_locs = []
    xmas_count = 0

    for i in range(1,len(data)-1):
        for j in range(1,len(data[i])-1):
            if data[i][j] == 'A':
                a_locs.append([i,j])
    
    for a in a_locs:
        i = a[0]
        j = a[1]
        up_left = data[i-1][j-1]
        up_right = data[i-1][j+1]
        down_left = data[i+1][j-1]
        down_right = data[i+1][j+1]

        if ((up_left == "M" and down_right == "S") or (up_left == "S" and down_right == "M")) and ((up_right == "M" and down_left == "S") or (up_right == "S" and down_left == "M")):
            xmas_count += 1


    print("Found", xmas_count, "xmases")


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('filename')

    args = parser.parse_args()

    data = import_data(args.filename)

    run_test(data)