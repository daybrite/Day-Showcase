# Copyright © The Daybrite Project
# SPDX-License-Identifier: MPL-2.0

# The Media page's cards (src/pages/media.rs).

media_clip = إعصار ميلتون من محطة الفضاء · NASA
media_clip_title = إعصار ميلتون من محطة الفضاء
media_clip_credit = NASA
media_played = تم تشغيله
media_buffered = مخزَّن مؤقتًا
media_buffered_unavailable = لا يبلّغ هذا التولكيت عن النطاقات المخزَّنة مؤقتًا.
media_playback_section = التشغيل
media_rate = السرعة
media_volume = مستوى الصوت
media_mute = كتم الصوت
media_volume_fixed = الفيديو على هذه المنصة يدعم كتم الصوت فقط.
media_system_section = عناصر تحكم النظام
media_now_playing = قيد التشغيل الآن
media_remote_title = آخر أمر
media_system_note = تعرض عناصر التحكم في الوسائط ومفاتيح الوسائط في النظام هذا الفيديو وتتحكم فيه.
media_readout_section = القراءات
media_stream_title = عنوان البث
media_error_title = الخطأ
media_none = لا يوجد
media_state = { $state ->
    [idle] خامل
    [loading] جارٍ التحميل
    [playing] قيد التشغيل
    [paused] متوقف مؤقتًا
    [ended] انتهى
   *[error] فشل
}
media_error = { $kind ->
    [invalid] قيمة غير صالحة: { $operation }
    [unsupported] غير مدعوم هنا: { $operation }
    [playback] فشل التشغيل
   *[command] فشل الأمر: { $operation }
}
media_remote = { $command ->
    [play] تشغيل
    [pause] إيقاف مؤقت
    [toggle] تشغيل أو إيقاف مؤقت
    [stop] إيقاف
    [next] التالي
    [previous] السابق
    [seek] انتقال
    [forward] تقديم
    [backward] ترجيع
   *[none] لا شيء
}
