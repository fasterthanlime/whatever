
extern void do_whatever(char *c, size_t len);

typedef void (*call_me_maybe_callback_t)(size_t);

extern void call_me_maybe(call_me_maybe_callback_t cb);

//

typedef struct RString RString;

extern char *rstring_data(RString *s);
extern size_t rstring_len(RString *s);
extern void rstring_free(RString *s);

//

extern RString *returns_whatever(void);

