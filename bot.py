import getHours as gh
import discord
client=discord.Client()
from random import randint
import time
import sys
import subprocess
import os
import codecs

@client.event
async def on_ready():
    print('Logged in')
    print("Username: ",end='')
    print(client.user.name)
    print("Userid: ",end='')
    print(client.user.id)
    await client.change_presence(activity=discord.Activity(type=discord.ActivityType.watching, name="i'm a rusty bot"))



@client.event
async def on_message(message):

    users = {}

    with open("users.txt","r") as file:
        data = file.readlines()

    for line in data:
        sline = line.split(",")
        print(sline)
        if len(sline) < 2:
            pass
        else:
            users[sline[0]] = sline[1][:-1]

    if message.author.id == client.user.id:
        return


    if message.content.startswith("/hours"):

        parsed = message.content.split(" ")
        print(parsed, len(parsed))
        
    
        try:
            target = parsed[1]
        except expression as identifier:
            await message.channel.send(f"```please provide a target use /list to see a list of avaliable targets```")

        if target.lower() in users.keys():
            pass
        else:
            await message.channel.send(f"not a target , run /list for a list of targets")
        
        hoursPlayed = gh.get(users[target.lower()])
        hoursInWeek = 14*24
        percent = (float(hoursPlayed)/hoursInWeek)*100

        await message.channel.send(f"{target} has played gaymes \n **`played: {hoursPlayed}hrs \npercent: {percent}% `** \n in the last 2 weeks".format(message))


    if message.content == "/list":
        out = ""

        with open("users.txt","r") as file:
            data = file.readlines()

        for name in data:
            out = out + name.lower() + "\n"

        await message.channel.send(f"```{out}```")
    


    if message.content == "/rand":
        out = subprocess.check_output(['./MultiThreadedWebScraper/target/release/MultiThreadedWebScraper'])
        os.system("rm -R /tmp/MTWS")

        out = codecs.decode(out, 'unicode_escape')
        await message.channel.send(out)


            


with open("token.txt") as file:
    token = file.read()

client.run(token)
