#This was the script I used to generate the character rates

import csv
attack_ids = [
    "Grab",
    "Taunt/Item Throw",
    "Jab 1",
    "Jab 2",
    "Jab 3",
    "Rapid Jabs",
    "Dash Attack",
    "Side Tilt",
    "Up Tilt",
    "Down Tilt",
    "Side Smash",
    "Up Smash",
    "Down Smash",
    "Nair",
    "Fair",
    "Bair",
    "Uair",
    "Dair",
    "Neutral Special",
    "Side Special",
    "Up Special",
    "Down Special",
    "Kirby Hat: Mario Neutral Special",
    "Kirby Hat: Fox Neutral Special",
    "Kirby Hat: CFalcon Neutral Special",
    "Kirby Hat: DK Neutral Special",
    "Kirby Hat: Bowser Neutral Special",
    "Kirby Hat: Link Neutral Special",
    "Kirby Hat: Sheik Neutral Special",
    "Kirby Hat: Ness Neutral Special",
    "Kirby Hat: Peach Neutral Special",
    "Kirby Hat: Ice Climber Neutral Special",
    "Kirby Hat: Pikachu Neutral Special",
    "Kirby Hat: Samus Neutral Special",
    "Kirby Hat: Yoshi Neutral Special",
    "Kirby Hat: Jigglypuff Neutral Special",
    "Kirby Hat: Mewtwo Neutral Special",
    "Kirby Hat: Luigi Neutral Special",
    "Kirby Hat: Marth Neutral Special",
    "Kirby Hat: Zelda Neutral Special",
    "Kirby Hat: Young Link Neutral Special",
    "Kirby Hat: Doc Neutral Special",
    "Kirby Hat: Falco Neutral Special",
    "Kirby Hat: Pichu Neutral Special",
    "Kirby Hat: Game & Watch Neutral Special",
    "Kirby Hat: Ganon Neutral Special",
    "Kirby Hat: Roy Neutral Special",
    "Unk",
    "Unk",
    "Unk",
    "Get Up Attack (From Back)",
    "Get Up Attack (From Front)",
    "Pummel",
    "Forward Throw",
    "Back Throw",
    "Up Throw",
    "Down Throw",
    "Cargo Forward Throw",
    "Cargo Back Throw",
    "Cargo Up Throw",
    "Cargo Down Throw",
    "Ledge Get Up Attack 100%+",
    "Ledge Get Up Attack",
    "Beam Sword Jab",
    "Beam Sword Tilt Swing",
    "Beam Sword Smash Swing",
    "Beam Sword Dash Swing",
    "Home Run Bat Jab",
    "Home Run Bat Tilt Swing",
    "Home Run Bat Smash Swing",
    "Home Run Bat Dash Swing",
    "Parasol Jab",
    "Parasol Tilt Swing",
    "Parasol Smash Swing",
    "Parasol Dash Swing",
    "Fan Jab",
    "Fan Tilt Swing",
    "Fan Smash Swing",
    "Fan Dash Swing",
    "Star Rod Jab",
    "Star Rod Tilt Swing",
    "Star Rod Smash Swing",
    "Star Rod Dash Swing",
    "Lip's Stick Jab",
    "Lip's Stick Tilt Swing",
    "Lip's Stick Smash Swing",
    "Lip's Stick Dash Swing",
    "Open Parasol",
    "Ray Gun Shoot",
    "Fire Flower Shoot",
    "Screw Attack",
    "Super Scope (Rapid)",
    "Super Scope (Charged)",
    "Hammer"
]

character_ids = [
    "Mario",
    "Fox",
    "CaptainFalcon",
    "DonkeyKong",
    "Kirby",
    "Bowser",
    "Link",
    "Sheik",
    "Ness",
    "Peach",
    "IceClimbers",
    "Nana",
    "Pikachu",
    "Samus",
    "Yoshi",
    "Jigglypuff",
    "Mewtwo",
    "Luigi",
    "Marth",
    "Zelda",
    "YoungLink",
    "DrMario",
    "Falco",
    "Pichu",
    "GameAndWatch",
    "Ganondorf",
    "Roy",
    "MasterHand",
    "CrazyHand",
    "WireFrameMale",
    "WireFrameFemale",
    "GigaBowser",
    "Sandbag"
]

# Dictionary to store counts
true_combo_count_dict = {}
skip = {}
# Open the CSV file
with open('combos.csv', 'r') as file:
    # Create a CSV reader object
    reader = csv.reader(file)

    # Skip the header row
    next(reader)

    # Iterate over each row in the CSV file
    for row in reader:
        # Each row is a list containing the values in the CSV row
        #row[4] is the move, row[6] is the character
        key = row[6]
        
        startMove=row[4]
        endMove=row[5]
        comboerCharacter=row[6]
        #if row[4]=='56' and row[6]=='8':
        #    print(f"Ness Dthrow combos into     {attack_ids[int(row[5])]:<{15}} on {character_ids[int(row[7])]:<{15}} at {int(float(row[8]))}%")

        if (startMove=='0' or startMove=='52') and comboerCharacter!='10':
            #grab/pummel?? (turns out to be items unless the comboer is icies)
            #print(f"{character_ids[int(row[6])]:<{15}} Grab (??) combos into     {attack_ids[int(row[5])]:<{15}} on {character_ids[int(row[7])]:<{15}} at {int(float(row[8]))}%")
            skip[(row[4], row[6])]=skip.get((row[4], row[6]),0)-1
            continue

        if  (startMove=='2' and endMove=='2') or (startMove=='3' and endMove=='2') or (startMove=='2' and endMove=='3') or (startMove=='3' and endMove=='4') or (startMove=='4' and endMove=='5') or (startMove=='5' and endMove=='5') or (startMove=='18' and endMove=='18' and comboerCharacter=='5') or (startMove=='19' and endMove=='19' and comboerCharacter=='8') or (startMove=='9' and endMove=='9' and comboerCharacter=='8') or (startMove=='10' and endMove=='10' and comboerCharacter=='20') or (startMove=='21' and endMove=='21' and comboerCharacter=='23') or (startMove=='21' and comboerCharacter=='7') or (startMove=='19' and endMove=='19' and (comboerCharacter=='26' or comboerCharacter=='18')):
            #skip jab combos, Bowser's fire, Ness's fire, Ness's dtilt, Young Link's Fsmash, Pichu's thunder, Sheik's transform (??), dancing blade
            skip[(row[4], row[6])]=skip.get((row[4], row[6]),0)-1
            continue

        
        # Increment the count for the current key
        true_combo_count_dict[key] = true_combo_count_dict.get(key, 0) + 1
    
    hits_count_dict = {}

# Open the CSV file
with open('totals.csv', 'r') as file:
    # Create a CSV reader object
    reader = csv.reader(file)

    # Skip the header row
    next(reader)

    # Iterate over each row in the CSV file
    for row in reader:
        # Each row is a list containing the values in the CSV row
        #row[1] is the move, row[0] is the character
        key = row[0]
        
        # Increment the count for the current key
        hits_count_dict[key] = hits_count_dict.get(key,0) + int(row[2])+skip.get((row[1], row[0]), 0)

combo_rates = []
for key in hits_count_dict:
    combo_rate = true_combo_count_dict.get(key, 0)/hits_count_dict[key]
    #print("char: ",character_ids[int(key[1])],",\t\t move: ",attack_ids[int(key[0])],",\t\t rate: ",combo_rate)
    combo_rates.append((character_ids[int(key)],combo_rate,hits_count_dict[key]))

combo_rates = sorted(combo_rates, key=lambda x: x[1],reverse = True)

# Write the sorted array to the CSV file
with open("character_rates.csv", "w", newline="") as file:
    writer = csv.writer(file)
    writer.writerow(["Character","Combo Rate","Total Occurrences"])  # Write header
    for item in combo_rates:
        writer.writerow(item)
