import requests
import os



def parseLine(line):

    line = str(line)

    chars = line.split("<h2>")
    line = str(chars)

    chars = line.split(" ")

    charCount = 0;

    arr = []

    for char in chars:
        arr.append(char)

    return arr[1][1:]



def parse(lines):

    target = ""

    for line in lines:
        if "hours past 2 weeks" in line:
            return parseLine(line)


def genWebFile(steamID):
    os.system("rm "+steamID)
    os.system("wget -q https://steamcommunity.com/profiles/"+steamID)


def get(steamID):

    steamID = str(steamID)

    genWebFile(steamID)

    file = open(steamID,"r")
    website = file.readlines()


    #os.system("rm index*")
    os.system("rm "+steamID)
    return parse(website)


