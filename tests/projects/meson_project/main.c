#include<stdio.h>
#include<string.h>
#include<ctype.h>
#define T 100
#define B 5
#define M 6

char a[B]="-",b[B]=".",c[B]=" ";

void morser(char st,char *str1);

int main(int argc,char **argv){
    char flaga=1;
    char str[T]="",str1[B*M*T]="";
    for(int i=1;i<argc;i++){
        if((!strcmp(argv[i],"-a")&& i<=(argc-3))&&flaga){
            flaga=0;
            i++;
            strncpy(b,argv[i],B);
            i++;
            strncpy(a,argv[i],B);
            i++;
            strncpy(c,argv[i],B);
            i++;
        }
        if((!strcmp(argv[i],"-b")&& i<=(argc-1))&&flaga){
            flaga=0;
            i++;
            b[0]=argv[i][0];
            a[0]=argv[i][1];
            c[0]=argv[i][2];
        }
        else if (!strcmp(argv[i],"-h") || !strcmp(argv[i],"--help")){
            printf(
"tomorse: Text to morese\n\
tomorse wordy words\n\
tomorse -a symbol1 symbol2 symbol3  words words words\n\
tomorse -b -.sp words\n"
);
            return 0;
        }
        else {
            strcat(str, argv[i]);
        }
    }
    for(int i=0;i<=strlen(str);i++){
        morser(str[i],str1);
    }
    printf("%s\n",str1);
    return 0;
}

void morser(char st,char *str1){
    switch(toupper(st)){
    case 'A':
        strcat(str1,b);
        strcat(str1,a);
        break;

    case 'B':
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        break;

    case 'C':
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        break;

    case 'D':
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        break;

    case 'E':
        strcat(str1,b);
        break;

    case 'F':
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        break;

    case 'G':
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,b);
        break;

    case 'H':
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        break;

    case 'I':
        strcat(str1,b);
        strcat(str1,b);
        break;

    case 'J':
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,a);
        break;

    case 'K':
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,a);
        break;

    case 'L':
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
      strcat(str1,c);
        break;

    case 'M':
        strcat(str1,a);
        strcat(str1,a);
        break;

    case 'N':
        strcat(str1,a);
        strcat(str1,b);
        break;

    case 'O':
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,a);
        break;

    case 'P':
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,b);
        break;

    case 'Q':
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,a);
        break;

    case 'R':
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        break;

    case 'S':
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        break;

    case 'T':
        strcat(str1,a);
        break;

    case 'U':
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        break;

    case 'V':
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        break;

    case 'W':
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,a);
        break;

    case 'X':
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        break;

    case 'Y':
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,a);
        break;

    case 'Z':
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        break;

    case '0':
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,a);
        break;

    case '1':
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,a);
        break;

    case '2':
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,a);
        break;

    case '3':
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,a);
        break;

    case '4':
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        break;

    case '5':
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        break;

    case '6':
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        break;

    case '7':
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        break;

    case '8':
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        break;

    case '9':
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,b);
        break;

    case '.':
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,a);
        break;

    case ',':
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,a);
        break;

    case ':':
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        break;

    case '?':
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        break;


    case '-':
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        break;

    case ';':
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        break;

    case '"':
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        break;

    case '+':
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        break;

    case '/':
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        break;

    case '&':
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        break;

    case '$':
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        break;


    case '@':
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,a);
        strcat(str1,b);
        break;

    case '=':
        strcat(str1,a);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,b);
        strcat(str1,a);
        break;
    default:
        break;
    }
    strcat(str1,c);
}
