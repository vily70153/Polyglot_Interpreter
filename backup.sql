-- MySQL dump 10.13  Distrib 8.0.42, for Linux (x86_64)
--
-- Host: localhost    Database: kursach
-- ------------------------------------------------------
-- Server version	5.5.5-10.11.11-MariaDB

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `AllLexemsTBL`
--

DROP TABLE IF EXISTS `AllLexemsTBL`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `AllLexemsTBL` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `lang_name` varchar(64) NOT NULL,
  `lexem` varchar(128) NOT NULL,
  `type_info` varchar(128) DEFAULT NULL,
  `std_lexem` int(10) unsigned NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `lang_lexem_unique` (`lang_name`,`lexem`),
  KEY `std_lexem` (`std_lexem`),
  CONSTRAINT `AllLexemsTBL_ibfk_1` FOREIGN KEY (`std_lexem`) REFERENCES `StdLexemeTBL` (`id`) ON DELETE NO ACTION ON UPDATE NO ACTION
) ENGINE=InnoDB AUTO_INCREMENT=249 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `AllLexemsTBL`
--

LOCK TABLES `AllLexemsTBL` WRITE;
/*!40000 ALTER TABLE `AllLexemsTBL` DISABLE KEYS */;
INSERT INTO `AllLexemsTBL` VALUES (1,'UA','{',NULL,1),(2,'UA','}',NULL,2),(3,'UA','(',NULL,3),(4,'UA',')',NULL,4),(5,'UA','[',NULL,5),(6,'UA',']',NULL,6),(7,'UA',':',NULL,7),(8,'UA',';',NULL,8),(9,'UA',',',NULL,9),(10,'UA','.',NULL,10),(11,'UA','->',NULL,11),(12,'UA','=>',NULL,12),(13,'UA','+',NULL,13),(14,'UA','-',NULL,14),(15,'UA','*',NULL,15),(16,'UA','/',NULL,16),(17,'UA','%',NULL,17),(18,'UA','==',NULL,18),(19,'UA','!=',NULL,19),(20,'UA','<',NULL,20),(21,'UA','<=',NULL,21),(22,'UA','>',NULL,22),(23,'UA','>=',NULL,23),(24,'UA','&&',NULL,24),(25,'UA','||',NULL,25),(26,'UA','!',NULL,26),(27,'UA','&',NULL,27),(28,'UA','|',NULL,28),(29,'UA','^',NULL,29),(30,'UA','~',NULL,30),(31,'UA','<<',NULL,31),(32,'UA','>>',NULL,32),(33,'UA','=',NULL,33),(34,'UA','+=',NULL,34),(35,'UA','-=',NULL,35),(36,'UA','*=',NULL,36),(37,'UA','/=',NULL,37),(38,'UA','%=',NULL,38),(39,'UA','&=',NULL,39),(40,'UA','|=',NULL,40),(41,'UA','^=',NULL,41),(42,'UA','<<=',NULL,42),(43,'UA','>>=',NULL,43),(44,'UA','identifier',NULL,70),(45,'UA','int_literal',NULL,71),(46,'UA','float_literal',NULL,72),(47,'UA','string_literal',NULL,73),(48,'UA','char_literal',NULL,74),(49,'UA','bool_literal',NULL,75),(64,'EN','^',NULL,29),(65,'EN','^=',NULL,41),(66,'EN','-',NULL,14),(67,'EN','-=',NULL,35),(68,'EN','->',NULL,11),(69,'EN',',',NULL,9),(70,'EN',';',NULL,8),(71,'EN',':',NULL,7),(72,'EN','!',NULL,26),(73,'EN','!=',NULL,19),(74,'EN','.',NULL,10),(75,'EN','(',NULL,3),(76,'EN',')',NULL,4),(77,'EN','[',NULL,5),(78,'EN',']',NULL,6),(79,'EN','{',NULL,1),(80,'EN','}',NULL,2),(81,'EN','*',NULL,15),(82,'EN','*=',NULL,36),(83,'EN','/',NULL,16),(84,'EN','/=',NULL,37),(85,'EN','&',NULL,27),(86,'EN','&&',NULL,24),(87,'EN','&=',NULL,39),(88,'EN','%',NULL,17),(89,'EN','%=',NULL,38),(90,'EN','+',NULL,13),(91,'EN','+=',NULL,34),(92,'EN','<',NULL,20),(93,'EN','<<',NULL,31),(94,'EN','<<=',NULL,42),(95,'EN','<=',NULL,21),(96,'EN','=',NULL,33),(97,'EN','==',NULL,18),(98,'EN','=>',NULL,12),(99,'EN','>',NULL,22),(100,'EN','>=',NULL,23),(101,'EN','>>',NULL,32),(102,'EN','>>=',NULL,43),(103,'EN','|',NULL,28),(104,'EN','|=',NULL,40),(105,'EN','||',NULL,25),(106,'EN','~',NULL,30),(107,'EN','async',NULL,63),(108,'EN','await',NULL,64),(109,'EN','bool_literal',NULL,75),(110,'EN','break',NULL,56),(111,'EN','char_literal',NULL,74),(112,'EN','class',NULL,47),(113,'EN','continue',NULL,57),(114,'EN','else',NULL,45),(115,'EN','export',NULL,59),(116,'EN','false',NULL,67),(117,'EN','float_literal',NULL,72),(118,'EN','for',NULL,49),(119,'EN','function',NULL,46),(120,'EN','identifier',NULL,70),(121,'EN','if',NULL,44),(122,'EN','import',NULL,58),(123,'EN','int_literal',NULL,71),(124,'EN','let',NULL,53),(125,'EN','match',NULL,52),(126,'EN','mut',NULL,54),(127,'EN','null',NULL,68),(128,'EN','private',NULL,61),(129,'EN','protected',NULL,62),(130,'EN','public',NULL,60),(131,'EN','return',NULL,55),(132,'EN','string_literal',NULL,73),(133,'EN','struct',NULL,48),(134,'EN','switch',NULL,51),(135,'EN','true',NULL,66),(136,'EN','undefined',NULL,69),(137,'EN','while',NULL,50),(138,'EN','yield',NULL,65),(191,'UA','Якщо',NULL,44),(194,'UA','Інакше',NULL,45),(197,'UA','Функція',NULL,46),(200,'UA','Клас',NULL,47),(203,'UA','Структура',NULL,48),(206,'UA','Цикл',NULL,49),(209,'UA','Поки',NULL,50),(212,'UA','Перемикач',NULL,51),(215,'UA','Відповідність',NULL,52),(218,'UA','Змінна',NULL,53),(221,'UA','Змінний',NULL,54),(224,'UA','ідентифікатор',NULL,70),(227,'UA','ціле','Int',71),(228,'EN','int','Int',71),(230,'UA','дійсне','Float',72),(231,'EN','float','Float',72),(233,'UA','рядок','String',73),(234,'EN','string','String',73),(236,'UA','символ','Char',74),(237,'EN','char','Char',74),(239,'UA','булеве','Bool',75),(240,'EN','bool','Bool',75),(241,'EN','print',NULL,300),(242,'EN','input',NULL,301),(243,'EN','len',NULL,302),(244,'UA','друк',NULL,300),(245,'UA','ввід',NULL,301),(246,'UA','довжина',NULL,302),(247,'UA','повернути',NULL,55);
/*!40000 ALTER TABLE `AllLexemsTBL` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `StdLexemeTBL`
--

DROP TABLE IF EXISTS `StdLexemeTBL`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `StdLexemeTBL` (
  `id` int(10) unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(64) NOT NULL,
  `lexem_type` varchar(64) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `name` (`name`)
) ENGINE=InnoDB AUTO_INCREMENT=303 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `StdLexemeTBL`
--

LOCK TABLES `StdLexemeTBL` WRITE;
/*!40000 ALTER TABLE `StdLexemeTBL` DISABLE KEYS */;
INSERT INTO `StdLexemeTBL` VALUES (1,'{','Delimiter'),(2,'}','Delimiter'),(3,'(','Delimiter'),(4,')','Delimiter'),(5,'[','Delimiter'),(6,']','Delimiter'),(7,':','Delimiter'),(8,';','Delimiter'),(9,',','Delimiter'),(10,'.','Delimiter'),(11,'->','Delimiter'),(12,'=>','Delimiter'),(13,'+','Operator'),(14,'-','Operator'),(15,'*','Operator'),(16,'/','Operator'),(17,'%','Operator'),(18,'==','Operator'),(19,'!=','Operator'),(20,'<','Operator'),(21,'<=','Operator'),(22,'>','Operator'),(23,'>=','Operator'),(24,'&&','Operator'),(25,'||','Operator'),(26,'!','Operator'),(27,'&','Operator'),(28,'|','Operator'),(29,'^','Operator'),(30,'~','Operator'),(31,'<<','Operator'),(32,'>>','Operator'),(33,'=','Operator'),(34,'+=','Operator'),(35,'-=','Operator'),(36,'*=','Operator'),(37,'/=','Operator'),(38,'%=','Operator'),(39,'&=','Operator'),(40,'|=','Operator'),(41,'^=','Operator'),(42,'<<=','Operator'),(43,'>>=','Operator'),(44,'if','Keyword'),(45,'else','Keyword'),(46,'function','Keyword'),(47,'class','Keyword'),(48,'struct','Keyword'),(49,'for','Keyword'),(50,'while','Keyword'),(51,'switch','Keyword'),(52,'match','Keyword'),(53,'let','Keyword'),(54,'mut','Keyword'),(55,'return','Keyword'),(56,'break','Keyword'),(57,'continue','Keyword'),(58,'import','Keyword'),(59,'export','Keyword'),(60,'public','Keyword'),(61,'private','Keyword'),(62,'protected','Keyword'),(63,'async','Keyword'),(64,'await','Keyword'),(65,'yield','Keyword'),(66,'true','Keyword'),(67,'false','Keyword'),(68,'null','Keyword'),(69,'undefined','Keyword'),(70,'identifier','Identifier'),(71,'int_literal','Literal'),(72,'float_literal','Literal'),(73,'string_literal','Literal'),(74,'char_literal','Literal'),(75,'bool_literal','Literal'),(300,'print','NativeFunc'),(301,'input','NativeFunc'),(302,'len','NativeFunc');
/*!40000 ALTER TABLE `StdLexemeTBL` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `__diesel_schema_migrations`
--

DROP TABLE IF EXISTS `__diesel_schema_migrations`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `__diesel_schema_migrations` (
  `version` varchar(50) NOT NULL,
  `run_on` timestamp NOT NULL DEFAULT current_timestamp(),
  PRIMARY KEY (`version`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `__diesel_schema_migrations`
--

LOCK TABLES `__diesel_schema_migrations` WRITE;
/*!40000 ALTER TABLE `__diesel_schema_migrations` DISABLE KEYS */;
/*!40000 ALTER TABLE `__diesel_schema_migrations` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2025-12-14 19:15:26
