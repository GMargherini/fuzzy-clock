import sys
from datetime import datetime

hours_dict = {'it': ["mezzanotte","l'una","le due", "le tre", "le quattro", "le cinque", "le sei", "le sette", "le otto", "le nove", "le dieci", "le undici", "mezzogiorno"],
              'en': ["midnight","one","two","three","four","five","six","seven","eight","nine", "ten", "eleven", "noon"]}
minutes_dict = {'it':["", "e cinque", "e dieci", "e un quarto", "e venti", "e venticinque", "e mezza", "meno venticinque", "meno venti", "meno un quarto", "meno dieci", "meno cinque"],
                'en':["", "five past", "ten past", "quarter past", "twenty past", "twenty-five past", "half past", "twenty-five to", "twenty to", "quarter to", "ten to", "five to"]}
languages = ['it', 'en']

def main():
    language = 'en'
    for arg in sys.argv:
        match arg:
            case 'it':  language = 'it'
            case 'en'|'': language = 'en'
            case '-h': print_help()
    now = datetime.now()
    hours, minutes = now.hour, (now.minute+2.5)%60
    precision = int(60/len(minutes_dict[language]))
    h_index = hours%12
    m_index = int(minutes/precision)
    hour_text = hours_dict[language][h_index]
    minute_text = minutes_dict[language][m_index]
    if minutes > 34+2.5:
        hour_text = hours_dict[language][(h_index+1)%12]
        if hours == 11:
            hour_text = hours_dict[language][12]
    elif hours == 12:
        hour_text = hours_dict[language][12]
    print_time(language,hour_text,minute_text)
 
def print_time(language, hour_text, minute_text):
    match language:
        case 'it': print(f'{hour_text} {minute_text}')
        case 'en': print(f'{minute_text} {hour_text}')

def print_help():
    print('Usage: python fuzzy_clock.py [OPTIONS] [LANGUAGE]')
    print('[OPTIONS]:\t-h:\tshow this help screen')
    print(f'[LANGUAGE]:\t{languages[0]}')
    for l in languages: 
        if not l == languages[0]:
            print(f'\t\t{l}') 
    sys.exit()

if __name__ == '__main__':
    main()