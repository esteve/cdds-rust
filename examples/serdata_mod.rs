struct sd {
    struct ddsi_serdata c;
    struct sampletype data;
    uint32_t keysz, pad0, valuesz, pad1;
  };
  
  static uint32_t sd_get_size (const struct ddsi_serdata *dcmn)
  {
    // FIXME: 4 is DDSI CDR header size and shouldn't be included here; same is true for pad1
    const struct sd *d = (const struct sd *) dcmn;
    return 4 + d->keysz + d->pad0 + d->valuesz + d->pad1;
  }
  
  static bool sd_eqkey (const struct ddsi_serdata *acmn, const struct ddsi_serdata *bcmn)
  {
    const struct sd *a = (const struct sd *) acmn;
    const struct sd *b = (const struct sd *) bcmn;
    return a->keysz == b->keysz && memcmp (a->data.key, b->data.key, a->keysz) == 0;
  }
  
  static void sd_free (struct ddsi_serdata *dcmn)
  {
    struct sd *d = (struct sd *) dcmn;
    free (d->data.key);
    free (d->data.value);
    free (d);
  }
  
  static char *strdup_with_len (const char *x, size_t l)
  {
    char *y = malloc (l);
    memcpy (y, x, l);
    return y;
  }
  
  static struct ddsi_serdata *sd_from_ser_iov (const struct ddsi_sertopic *tpcmn, enum ddsi_serdata_kind kind, ddsrt_msg_iovlen_t niov, const ddsrt_iovec_t *iov, size_t size)
  {
    struct stp const * const stp = (const struct stp *) tpcmn;
    struct sd *sd = malloc (sizeof (*sd));
    ddsi_serdata_init (&sd->c, &stp->c, kind);
    (void) size;
    // assuming not fragmented, CDR Header specifies native encoding, input is valid and all that
    // CDR encoding of strings: uint32_t length-including-terminating-0 ; blob-including-terminating-0
    assert (niov == 1);
    (void) niov;
    size_t off = 0;
    const char *base = (const char *) iov[0].iov_base + 4;
    memcpy (&sd->keysz, base + off, sizeof (uint32_t));
    off += sizeof (uint32_t);
    sd->data.key = strdup_with_len (base + off, sd->keysz);
    off += sd->keysz;
    sd->pad0 = ((off % 4) == 0) ? 0 : 4 - (off % 4);
    off += sd->pad0;
    if (kind == SDK_DATA)
    {
      memcpy (&sd->valuesz, base + off, sizeof (uint32_t));
      off += sizeof (uint32_t);
      sd->data.value = strdup_with_len (base + off, sd->valuesz);
      off += sd->valuesz;
      // FIXME: not sure if this is still needed, it shouldn't be, but ...
      sd->pad1 = ((off % 4) == 0) ? 0 : 4 - (off % 4);
    }
    else
    {
      sd->data.value = NULL;
      sd->valuesz = sd->pad1 = 0;
    }
    sd->c.hash = ddsrt_mh3 (sd->data.key, sd->keysz, 0) ^ tpcmn->serdata_basehash;
    return &sd->c;
  }
  
  static struct ddsi_serdata *sd_from_ser (const struct ddsi_sertopic *tpcmn, enum ddsi_serdata_kind kind, const struct nn_rdata *fragchain, size_t size)
  {
    assert (fragchain->nextfrag == NULL);
    ddsrt_iovec_t iov = {
      .iov_base = NN_RMSG_PAYLOADOFF (fragchain->rmsg, NN_RDATA_PAYLOAD_OFF (fragchain)),
      .iov_len = fragchain->maxp1 // fragchain->min = 0 for first fragment, by definition
    };
    return sd_from_ser_iov (tpcmn, kind, 1, &iov, size);
  }
  
  static struct ddsi_serdata *sd_from_keyhash (const struct ddsi_sertopic *tpcmn, const ddsi_keyhash_t *keyhash)
  {
    // unbounded string, therefore keyhash is MD5 and we can't extract the value
    // (don't try disposing/unregistering in RTI and expect this to accept the data)
    (void) tpcmn; (void) keyhash;
    return NULL;
  }
  
  static struct ddsi_serdata *sd_from_sample (const struct ddsi_sertopic *tpcmn, enum ddsi_serdata_kind kind, const void *sample)
  {
    const struct stp *tp = (const struct stp *) tpcmn;
    struct sd *sd = malloc (sizeof (*sd));
    const struct sampletype *s = sample;
    ddsi_serdata_init (&sd->c, &tp->c, kind);
    sd->keysz = (uint32_t) strlen (s->key) + 1;
    sd->data.key = strdup_with_len (s->key, sd->keysz);
    sd->pad0 = ((sd->keysz % 4) == 0) ? 0 : 4 - (sd->keysz % 4);
    if (kind == SDK_DATA)
    {
      sd->valuesz = (uint32_t) strlen (s->value) + 1;
      sd->data.value = strdup_with_len (s->value, sd->valuesz);
      sd->pad1 = ((sd->valuesz % 4) == 0) ? 0 : 4 - (sd->valuesz % 4);
    }
    else
    {
      sd->data.value = NULL;
      sd->valuesz = sd->pad1 = 0;
    }
    sd->c.hash = ddsrt_mh3 (sd->data.key, sd->keysz, 0) ^ tpcmn->serdata_basehash;
    return &sd->c;
  }
  
  static struct ddsi_serdata *sd_to_topicless (const struct ddsi_serdata *serdata_common)
  {
    const struct sd *sd = (const struct sd *) serdata_common;
    const struct stp *tp = (const struct stp *) sd->c.topic;
    struct sd *sd_tl = malloc (sizeof (*sd_tl));
    ddsi_serdata_init (&sd_tl->c, &tp->c, SDK_KEY);
    sd_tl->c.topic = NULL;
    sd_tl->c.hash = sd->c.hash;
    sd_tl->c.timestamp.v = INT64_MIN;
    sd_tl->data.key = strdup_with_len (sd->data.key, sd->keysz);
    sd_tl->data.value = NULL;
    sd_tl->keysz = sd->keysz;
    sd_tl->pad0 = sd->pad0;
    sd_tl->valuesz = sd_tl->pad1 = 0;
    return &sd_tl->c;
  }
  
  static struct ddsi_serdata *sd_to_ser_ref (const struct ddsi_serdata *serdata_common, size_t cdr_off, size_t cdr_sz, ddsrt_iovec_t *ref)
  {
    // The idea is that if the CDR is available already, one can return a reference (in which case
    // one probably should also increment the sample's refcount).  But here we generaete the CDR
    // on the fly
    const struct sd *sd = (const struct sd *) serdata_common;
    // fragmented data results in calls with different offsets and limits, but can't be bothered for this test
    assert (cdr_off == 0 && cdr_sz == sd_get_size (&sd->c));
    (void) cdr_off;
    (void) cdr_sz;
    ref->iov_len = sd_get_size (&sd->c);
    ref->iov_base = malloc (ref->iov_len);
    char * const header = ref->iov_base;
    header[0] = 0;
    header[1] = (DDSRT_ENDIAN == DDSRT_LITTLE_ENDIAN) ? 1 : 0;
    header[2] = 0;
    header[3] = 0;
    char * const base = header + 4;
    size_t off = 0;
    memcpy (base + off, &sd->keysz, sizeof (uint32_t));
    off += sizeof (uint32_t);
    memcpy (base + off, sd->data.key, sd->keysz);
    off += sd->keysz;
    for (uint32_t i = 0; i < sd->pad0; i++)
      base[off++] = 0;
    if (sd->c.kind == SDK_DATA)
    {
      memcpy (base + off, &sd->valuesz, sizeof (uint32_t));
      off += sizeof (uint32_t);
      memcpy (base + off, sd->data.value, sd->valuesz);
      off += sd->valuesz;
      for (uint32_t i = 0; i < sd->pad0; i++)
        base[off++] = 0;
    }
    return (struct ddsi_serdata *) &sd->c;
  }
  
  static void sd_to_ser_unref (struct ddsi_serdata *serdata_common, const ddsrt_iovec_t *ref)
  {
    // const "ref" is a mistake in the interface ... it is owned by this code ...
    (void) serdata_common;
    free ((void *) ref->iov_base);
  }
  
  static void sd_to_ser (const struct ddsi_serdata *serdata_common, size_t off, size_t sz, void *buf)
  {
    // being lazy and reusing code in sd_to_ser_ref, even though here we don't have to allocate anything
    ddsrt_iovec_t iov;
    (void) sd_to_ser_ref (serdata_common, off, sz, &iov);
    memcpy (buf, iov.iov_base, iov.iov_len);
    sd_to_ser_unref ((struct ddsi_serdata *) serdata_common, &iov);
  }
  
  static bool sd_to_sample (const struct ddsi_serdata *serdata_common, void *sample, void **bufptr, void *buflim)
  {
    const struct sd *sd = (const struct sd *) serdata_common;
    struct sampletype *s = sample;
    if (bufptr) abort(); else { (void)buflim; } /* FIXME: haven't implemented that bit yet! */
    s->key = dds_realloc (s->key, sd->keysz);
    memcpy (s->key, sd->data.key, sd->keysz);
    if (sd->c.kind == SDK_DATA)
    {
      s->value = dds_realloc (s->value, sd->valuesz);
      memcpy (s->value, sd->data.value, sd->valuesz);
    }
    return true;
  }
  
  static bool sd_topicless_to_sample (const struct ddsi_sertopic *topic, const struct ddsi_serdata *serdata_common, void *sample, void **bufptr, void *buflim)
  {
    (void) topic;
    const struct sd *sd = (const struct sd *) serdata_common;
    struct sampletype *s = sample;
    if (bufptr) abort(); else { (void)buflim; } /* FIXME: haven't implemented that bit yet! */
    s->key = dds_realloc (s->key, sd->keysz);
    memcpy (s->key, sd->data.key, sd->keysz);
    return true;
  }
  
  static size_t sd_print (const struct ddsi_sertopic *sertopic_common, const struct ddsi_serdata *serdata_common, char *buf, size_t size)
  {
    (void) sertopic_common;
    const struct sd *sd = (const struct sd *) serdata_common;
    int cnt;
    if (sd->c.kind == SDK_DATA)
      cnt = snprintf (buf, size, "%s -> %s", sd->data.key, sd->data.value);
    else
      cnt = snprintf (buf, size, "%s", sd->data.key);
    return ((size_t) cnt > size) ? size : (size_t) cnt;
  }
  
  static void sd_get_keyhash (const struct ddsi_serdata *serdata_common, struct ddsi_keyhash *buf, bool force_md5)
  {
    (void) force_md5; // unbounded string is always MD5
    struct sd const * const sd = (const struct sd *) serdata_common;
    const uint32_t keysz_be = ddsrt_toBE4u (sd->keysz);
    ddsrt_md5_state_t md5st;
    ddsrt_md5_init (&md5st);
    ddsrt_md5_append (&md5st, (ddsrt_md5_byte_t *) &keysz_be, sizeof (keysz_be));
    ddsrt_md5_append (&md5st, (ddsrt_md5_byte_t *) sd->data.key, sd->keysz);
    ddsrt_md5_finish (&md5st, (ddsrt_md5_byte_t *) (buf->value));
  }
  
  static const struct ddsi_serdata_ops sd_ops = {
    .get_size = sd_get_size,
    .eqkey = sd_eqkey,
    .free = sd_free,
    .from_ser = sd_from_ser,
    .from_ser_iov = sd_from_ser_iov,
    .from_keyhash = sd_from_keyhash,
    .from_sample = sd_from_sample,
    .to_ser = sd_to_ser,
    .to_sample = sd_to_sample,
    .to_ser_ref = sd_to_ser_ref,
    .to_ser_unref = sd_to_ser_unref,
    .to_topicless = sd_to_topicless,
    .topicless_to_sample = sd_topicless_to_sample,
    .print = sd_print,
    .get_keyhash = sd_get_keyhash
  };
  