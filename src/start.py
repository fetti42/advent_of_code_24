import argparse


def import_data(filename):
    data = [i.strip('\n') for i in open(filename.readlines())]
    return(data)




if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument('filename')

    args = parser.parse_args()

    data = import_data(args.filename)