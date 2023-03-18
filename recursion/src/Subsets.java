public class Subsets {
    

    private static  void subsets( String soFar, String rest)  {
        if(rest.equals(""))  {
            System.out.println(soFar);
            return;
        }

        // subsets including first character 
        subsets(soFar+rest.charAt(0), rest.substring(1));
        // subsets exceluding first character 
        subsets(soFar, rest.substring(1));

    }


    public static void main(String arg[])   {

           subsets("", "ABCD");
    }
}
