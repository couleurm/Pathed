# 🪃 Pathed

Search any website instantly only using your keyboard, like DuckDuckGo [!bangs](https://duckduckgo.com/bangs)



https://github.com/user-attachments/assets/abd070dd-9d5a-4d42-87d5-ab46b935eb7a

`s`, `ys`, `gn` are shortcuts I made that use Pathed to easily search anything online

# Setup

### 1. Make a folder to host Pathed and it's shortcuts, and add it to PATH

```PowerShell
if (-not(Test-Path C:\Pathed)){mkdir C:\Pathed}
if ($env:PATH -notlike "*C:\Pathed*"){[Environment]::SetEnvironmentVariable("Path", $env:PATH + ";C:\Pathed", "Machine")}
```

### 2. Download pathed.exe to the folder

Save it yourself from https://github.com/couleurm/Pathed/releases/latest/download/pathed.exe

or
```PowerShell
irm https://github.com/couleurm/Pathed/releases/latest/download/pathed.exe -out C:\pathed\pathed.exe
```

### 3. Make shortcuts

In order to make a shortcut, you need to create to decide on a shortcut filename that you'll remember to type in windows+r when you'll want to summon it, and the website you want to search with it's parameter syntax

e.g. when searching foo on youtube the url is `https://www.youtube.com/results?search_query=foo`, it's easy to assume you just need to remove foo and keep the rest

1. Open the C:\Pathed folder
2. Make a right click somewhere in the folder
3. New -> Shortcut
4. In the location of the item, type `pathed -w https://www.youtube.com/results?search_query= -q` (-w for website and -q for query)
5. Name the shortcut, e.g. `ys`, which stands for YouTube Search
6. Click finish to create it
7. Right click the shortcut and click properties
8. Set the Run field to Minimised so you don't see the console host flashing

### 4. Extra: make folder shortcuts

I've teached you how to make shortcuts to pathed.exe and easily summon them from windows+r, but you can also do this with folder, the best example that I recommend you do is to create a shortcut to open the C:\Pathed folder

1. Make a shortcut like in the previous few first steps
2. In the location of the item type the path to the folder, e.g. `C:\Pathed`
3. Name the shortcut, e.g. `pd`
4. Finish
