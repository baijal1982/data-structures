public class Anagram {


    private static boolean isAnagram(String soFar, String remaining , String word)  {

        //base case 
        if( remaining=="")  {
            return soFar.equals(word);
        }

        for( int count =0; count< remaining.length(); count++)   {
            String next =  soFar+ remaining.charAt(count);
            String left =  remaining.substring(0, count)+ remaining.substring(count+1);
            if(isAnagram(next , left, word) )  {
                return true;
            }
        }
        return false ;
    }

    public static void main(String arg[] )   {
              System.out.println(isAnagram("","listen","silent"));
    }
    
}
