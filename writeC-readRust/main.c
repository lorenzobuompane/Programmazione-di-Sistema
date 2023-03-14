#include <stdio.h>
#include <stdlib.h>

typedef struct {
    int type;
    float val;
    long timestamp;
} ValueStruct;

typedef struct {
    int type;
    float val[10];
    long timestamp;
} MValueStruct;

typedef struct {
    int type;
    char message[21]; // stringa null terminated lung max 20
} MessageStruct;

typedef struct {
    int type;
    union {
        ValueStruct val;
        MValueStruct mvals;
        MessageStruct messages;
    };
} ExportData;

void export(ExportData *data, int n, FILE *pf);

int main() {
    ExportData vec[100];
    for (int i=0; i<100; i++) {
        vec[i].type = rand()%3 + 1;
        switch (vec[i].type) {
            case 1: {       // VALUE
                vec[i].val.type=1;
                vec[i].val.val=(float) rand();
                vec[i].val.timestamp=(long) rand();
                break;
            }
            case 2: {       // MVALUE
                vec[i].mvals.type=2;
                for (int j=0; j<10; j++) {
                    vec[i].mvals.val[j]=(float) rand();
                }
                vec[i].mvals.timestamp=(long) rand();
                break;
            }
            case 3: {       // MESSAGE
                vec[i].messages.type=3;
                sprintf(vec[i].messages.message, "Hello World: %d\n", rand()%10);
                break;
            }
        }
    }
    FILE *pf = fopen("input_for_rust.bin", "wb");
    if (pf == NULL) {
        printf("Error in opening file!\n");
        return -1;
    }
    printf("main pre export\n");
    export(vec, 100, pf);
    printf("main post export\n");
    return 0;
}

void export(ExportData *data, int n, FILE *pf) {
    printf("export\n");
    for (int i=0; i<n; i++){
        fwrite(&data[i], sizeof(ExportData), 1, pf);
        /*switch (data[i].type) {
            case 1: {       // VALUE
                fwrite(&data[i].val, sizeof(ValueStruct), 1, pf);
                break;
            }
            case 2: {       // MVALUE
                fwrite(&data[i].mvals, sizeof(MValueStruct), 1, pf);
                break;
            }
            case 3: {       // MESSAGE
                fwrite(&data[i].messages, sizeof(MessageStruct), 1, pf);
                break;
            }
        }*/
    }
}