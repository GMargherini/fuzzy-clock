fn getTime(){
	time.
}
private fun getTime(): String {
        val sdf = SimpleDateFormat("mm")
        val minutes = sdf.format(Date()).toInt()
        sdf.applyPattern("h")
        val hour = sdf.format(Date()).toInt()
        val hours= arrayOf(
            getString(R.string.midnight),
            getString(R.string.one),
            getString(R.string.two),
            getString( R.string.three),
            getString(R.string.four),
            getString(R.string.five),
            getString(R.string.six),
            getString(R.string.seven),
            getString(R.string.eight),
            getString(R.string.nine),
            getString(R.string.ten),
            getString(R.string.eleven),
            getString(R.string.noon)
        )
        var hText = "hour"
        var mText = "minutes"
        var time ="$hText $mText"
        when(minutes){
            in 0..5 -> mText = ""
            in 6..12 -> mText = getString(R.string.tenPast)
            in 13..19 -> mText = getString(R.string.quarterPast)
            in 20..27 -> mText = getString(R.string.twentyPast)
            in 28..35 -> mText = getString(R.string.halfPast)
            in 36..42 -> mText = getString(R.string.twentyTo)
            in 43..49 -> mText = getString(R.string.quarterTo)
            in 50..56 -> mText = getString(R.string.tenTo)
            in 57..59 -> mText = getString(R.string.almost)
        }

        when(hour){
            0 -> hText = hours[0]
            1 -> hText = hours[1]
            2 -> hText = hours[2]
            3 -> hText = hours[3]
            4 -> hText = hours[4]
            5 -> hText = hours[5]
            6 -> hText = hours[6]
            7 -> hText = hours[7]
            8 -> hText = hours[8]
            9 -> hText = hours[9]
            10 -> hText = hours[10]
            11 -> hText = hours[11]
            12 -> hText = hours[12]
        }
        if(mText in arrayOf(getString(R.string.almost),getString(R.string.tenTo),getString(R.string.quarterTo),getString(R.string.twentyTo))){
                hText = hours[(hour+1)%13]
        }
        when(Locale.getDefault().language){
            "it" -> time = if (mText == getString(R.string.almost)){
                "${mText.replaceFirstChar { if (it.isLowerCase()) it.titlecase(Locale.ROOT) else it.toString() }} ${hText.lowercase()}"
            } else{
                "${hText.replaceFirstChar { if (it.isLowerCase()) it.titlecase(Locale.ROOT) else it.toString() }} ${mText.lowercase()}"
            }
            "en" -> time = "${mText.replaceFirstChar { if (it.isLowerCase()) it.titlecase(Locale.ROOT) else it.toString() }} ${hText.lowercase()}"
        }
        return time
    }
    private fun getDate(): String {
        val sdf = SimpleDateFormat.getDateInstance()
        return sdf.format(Date())
    }